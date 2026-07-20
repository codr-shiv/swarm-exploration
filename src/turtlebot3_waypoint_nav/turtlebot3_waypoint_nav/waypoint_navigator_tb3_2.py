#!/usr/bin/env python3
"""
Waypoint Navigator for TurtleBot3 — custom_arena (5x5 world)
=====================================================
- Reads hardcoded waypoints (x, y) defined below
- For each waypoint:
    1. Rotates in place to face the target (yaw correction)
    2. Moves forward until within POSITION_TOLERANCE metres
    3. Does a full 360-degree spin so SLAM gets a complete scan
- Uses /odom for real position feedback (not pure time-based)
- Publishes to /cmd_vel (Twist)

Tunable constants are at the top — change them without touching logic.
"""

import math
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
from rclpy.qos import QoSProfile, ReliabilityPolicy, HistoryPolicy


# ─────────────────────────────────────────────
#  TUNABLE CONSTANTS  — edit these freely
# ─────────────────────────────────────────────

# Velocity limits
LINEAR_VELOCITY   = 0.26   # m/s   — keep low (0.1–0.2) for clean SLAM scans
ANGULAR_VELOCITY  = 0.4    # rad/s — rotation speed (both turning & 360 spin)

# Tolerances
POSITION_TOLERANCE = 0.15  # metres — how close is "arrived"
ANGLE_TOLERANCE    = 0.05  # radians (~3°) — how aligned before moving forward

# 360-degree spin
SPIN_RATE          = 0.5   # rad/s — speed of the 360 spin at each waypoint
FULL_ROTATION      = 2 * math.pi  # radians

# Waypoints — (x, y) in metres, world frame
# Chosen to cover all 4 obstacle areas + corners of the 5x5 arena.
# Robot spawns at (-2.0, -2.0) — waypoints start near there.
# Modify freely — keep values inside [-2.3, 2.3] to stay inside walls.
WAYPOINTS = [
    (2, 2),   # near spawn, open corner
    (-2,2),
    (-2,-2),
    (2,-2),
    (2,2),
    (0,2),
    (1,1),
    (-1,1),
    (-1,-1),
    (1,-1),
    (1,1),
]


# ─────────────────────────────────────────────
#  HELPER — extract yaw from quaternion
# ─────────────────────────────────────────────

def quaternion_to_yaw(q):
    """Convert ROS quaternion to yaw angle (radians)."""
    siny_cosp = 2.0 * (q.w * q.z + q.x * q.y)
    cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z)
    return math.atan2(siny_cosp, cosy_cosp)


def angle_diff(a, b):
    """Shortest signed difference between two angles (radians)."""
    diff = a - b
    while diff >  math.pi: diff -= 2 * math.pi
    while diff < -math.pi: diff += 2 * math.pi
    return diff


# ─────────────────────────────────────────────
#  MAIN NODE
# ─────────────────────────────────────────────

class WaypointNavigator(Node):

    def __init__(self):
        super().__init__('waypoint_navigator')

        qos = QoSProfile(
            reliability=ReliabilityPolicy.BEST_EFFORT,
            history=HistoryPolicy.KEEP_LAST,
            depth=10
        )

        self.cmd_pub = self.create_publisher(Twist, 'TB3_2/cmd_vel', 10)
        self.odom_sub = self.create_subscription(
            Odometry, 'TB3_2/odom', self.odom_callback, qos)

        # Current pose (updated by odom callback)
        self.x   = 0.0
        self.y   = 0.0
        self.yaw = 0.0
        self.odom_received = False

        # 10 Hz control loop
        self.timer = self.create_timer(0.1, self.control_loop)

        # State machine
        self.waypoint_index = 0
        # States: 'wait_odom' → 'rotate_to_goal' → 'move_forward'
        #         → 'spin_360' → next waypoint or 'done'
        self.state = 'wait_odom'
        self.spin_accumulated = 0.0
        self.spin_last_yaw    = None

        self.get_logger().info(
            f'WaypointNavigator ready. {len(WAYPOINTS)} waypoints loaded.')
        self.get_logger().info(
            f'LINEAR_VELOCITY={LINEAR_VELOCITY} m/s  '
            f'ANGULAR_VELOCITY={ANGULAR_VELOCITY} rad/s  '
            f'POSITION_TOLERANCE={POSITION_TOLERANCE} m')

    # ── Odometry callback ──────────────────────────────────────────────

    def odom_callback(self, msg):
        self.x   = msg.pose.pose.position.x
        self.y   = msg.pose.pose.position.y
        self.yaw = quaternion_to_yaw(msg.pose.pose.orientation)
        self.odom_received = True

    # ── Publish helper ─────────────────────────────────────────────────

    def publish_vel(self, linear=0.0, angular=0.0):
        t = Twist()
        t.linear.x  = linear
        t.angular.z = angular
        self.cmd_pub.publish(t)

    def stop(self):
        self.publish_vel(0.0, 0.0)

    # ── Main control loop (10 Hz) ──────────────────────────────────────

    def control_loop(self):

        # ── wait for first odom message ──
        if self.state == 'wait_odom':
            if self.odom_received:
                self.get_logger().info('Odometry received. Starting navigation.')
                self.state = 'rotate_to_goal'
            return

        # ── all waypoints done ──
        if self.state == 'done':
            self.stop()
            return

        # ── current target ──
        if self.waypoint_index >= len(WAYPOINTS):
            self.get_logger().info('All waypoints visited. Stopping.')
            self.stop()
            self.state = 'done'
            return

        goal_x, goal_y = WAYPOINTS[self.waypoint_index]
        dx = goal_x - self.x
        dy = goal_y - self.y
        distance  = math.hypot(dx, dy)
        target_yaw = math.atan2(dy, dx)

        # ────────────────────────────────
        # STATE 1: rotate to face goal
        # ────────────────────────────────
        if self.state == 'rotate_to_goal':
            err = angle_diff(target_yaw, self.yaw)
            if abs(err) > ANGLE_TOLERANCE:
                direction = 1.0 if err > 0 else -1.0
                self.publish_vel(angular=direction * ANGULAR_VELOCITY)
            else:
                self.stop()
                self.get_logger().info(
                    f'Facing waypoint {self.waypoint_index} '
                    f'({goal_x:.2f}, {goal_y:.2f}). Moving forward.')
                self.state = 'move_forward'

        # ────────────────────────────────
        # STATE 2: move forward to goal
        # ────────────────────────────────
        elif self.state == 'move_forward':
            if distance > POSITION_TOLERANCE:
                # Small heading correction while moving
                heading_err = angle_diff(target_yaw, self.yaw)
                correction  = 0.5 * heading_err  # proportional, gentle
                self.publish_vel(
                    linear=LINEAR_VELOCITY,
                    angular=correction)
            else:
                self.stop()
                self.get_logger().info(
                    f'Arrived at waypoint {self.waypoint_index} '
                    f'({goal_x:.2f}, {goal_y:.2f}). '
                    f'Distance error: {distance:.3f} m. Starting 360 spin.')
                self.spin_accumulated = 0.0
                self.spin_last_yaw    = self.yaw
                self.state = 'spin_360'

        # ────────────────────────────────
        # STATE 3: 360-degree spin
        # ────────────────────────────────
        elif self.state == 'spin_360':
            if self.spin_last_yaw is None:
                self.spin_last_yaw = self.yaw

            delta = abs(angle_diff(self.yaw, self.spin_last_yaw))
            self.spin_accumulated += delta
            self.spin_last_yaw     = self.yaw

            if self.spin_accumulated < FULL_ROTATION:
                self.publish_vel(angular=SPIN_RATE)
            else:
                self.stop()
                self.get_logger().info(
                    f'360 spin done at waypoint {self.waypoint_index}. '
                    f'Total rotated: {math.degrees(self.spin_accumulated):.1f} deg.')
                self.waypoint_index += 1
                self.state = 'rotate_to_goal'


# ─────────────────────────────────────────────
#  ENTRY POINT
# ─────────────────────────────────────────────

def main(args=None):
    rclpy.init(args=args)
    node = WaypointNavigator()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        node.get_logger().info('Interrupted by user.')
    finally:
        node.stop()
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
