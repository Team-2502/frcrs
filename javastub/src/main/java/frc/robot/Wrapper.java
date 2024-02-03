package frc.robot;

import com.ctre.phoenix.motorcontrol.ControlMode;
import com.ctre.phoenix.motorcontrol.can.TalonFX;
import com.ctre.phoenix.motorcontrol.can.WPI_TalonFX;
import com.ctre.phoenix.sensors.CANCoder;
import com.kauailabs.navx.frc.AHRS;
import com.revrobotics.CANSparkMaxLowLevel.MotorType;
import com.revrobotics.CANSparkMax.IdleMode;
import com.revrobotics.CANSparkMax.ControlType;
import edu.wpi.first.networktables.NetworkTableInstance;

import static edu.wpi.first.wpilibj.RobotBase.isReal;

/// Wrapper to use rev classes in rust, for whatever reason they cannot be accesed from rust directly
public class Wrapper {
    public static MotorType kBrushless() {
        return MotorType.kBrushless;
    }
    public static MotorType kBrushed() {
        return MotorType.kBrushed;
    }

    public static IdleMode kBrake() { return IdleMode.kBrake; }
    public static IdleMode kCoast() { return IdleMode.kCoast; }

    public static ControlType kPosition() { return ControlType.kPosition; }

    public static void startNetworkTables() {
        NetworkTableInstance inst = NetworkTableInstance.getDefault();
        if (isReal()) {
            inst.startServer("/home/lvuser/networktables.json");
        } else {
            inst.startServer();
        }
    }

    public static AHRS createAHRS() {
        return new AHRS();
    }

    public static double getAngle(AHRS navx) {
        return navx.getAngle();
    }

    public static ControlMode ctreVelocity() {
        return ControlMode.Velocity;
    }

    public static ControlMode ctrePosition() {
        return ControlMode.Position;
    }

    //public static TalonFXInvertType TalonFXCounterClockwise() { return TalonFXInvertType.CounterClockwise; }
}
