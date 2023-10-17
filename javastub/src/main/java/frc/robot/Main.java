// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

package frc.robot;

import com.ctre.phoenix.motorcontrol.can.WPI_TalonFX;
import com.revrobotics.CANSparkMax;
import com.revrobotics.CANSparkMaxLowLevel;
import edu.wpi.first.hal.DriverStationJNI;
import edu.wpi.first.wpilibj.DriverStation;
import edu.wpi.first.wpilibj.Filesystem;
import edu.wpi.first.wpilibj.Joystick;
import edu.wpi.first.wpilibj.RobotBase;
import edu.wpi.first.wpilibj.smartdashboard.SmartDashboard;

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

  private static native void rustentry(); // stub for entrypoint
  static {
    System.load( // load rust static library
      Filesystem.getDeployDirectory() + "/libfrcrs.so"
    );
  }
  public static void main(String... args) {
    rustentry();
    //CANSparkMax spark = new CANSparkMax(5, CANSparkMaxLowLevel.MotorType.kBrushless);
    //com.revrobotics.CANSparkMaxLowLevel.MotorType.fromId(1);
    //DriverStation.reportWarning("In Teleop!", false);
    //DriverStationJNI.observeUserProgramStarting();
    //RobotBase.startRobot(Robot::new);
  }
}

