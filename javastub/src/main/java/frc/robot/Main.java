// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

package frc.robot;

<<<<<<< HEAD
import edu.wpi.first.wpilibj.Filesystem;
=======
import edu.wpi.first.wpilibj.RobotBase;
>>>>>>> 9f22ad9 (updated to 2025 javastub)

/**
 * Do NOT add any static variables to this class, or any initialization at all. Unless you know what
 * you are doing, do not modify this file except to change the parameter class to the startRobot
 * call.
 */
public final class Main {
  private Main() {}

  /**
   * Main initialization function. Do not perform any initialization here.
   *
   * <p>If you change your main robot class, change the parameter type.
   */
<<<<<<< HEAD

  private static native void rustentry(); // stub for entrypoint
  static {
    System.load( // load rust static library
      Filesystem.getDeployDirectory() + "/libRobotCode2025.so"
    );
  }
  public static void main(String... args) {
    rustentry();
  }
=======
  public static void main(String... args) {}
>>>>>>> 9f22ad9 (updated to 2025 javastub)
}
