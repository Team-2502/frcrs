package frc.robot;

import com.ctre.phoenix6.StatusSignal;
import com.revrobotics.spark.SparkBase;
import com.revrobotics.spark.SparkLowLevel;
import com.revrobotics.spark.config.SparkBaseConfig;
import com.studica.frc.AHRS;
import edu.wpi.first.hal.AllianceStationID;
import edu.wpi.first.hal.DriverStationJNI;
import edu.wpi.first.math.geometry.*;
import edu.wpi.first.networktables.NetworkTableInstance;
import edu.wpi.first.wpilibj.PneumaticsModuleType;
import edu.wpi.first.wpilibj.livewindow.LiveWindow;
import edu.wpi.first.wpilibj.shuffleboard.Shuffleboard;
import edu.wpi.first.wpilibj.smartdashboard.Field2d;
import edu.wpi.first.wpilibj.smartdashboard.SendableChooser;
import edu.wpi.first.wpilibj.smartdashboard.SmartDashboard;
import edu.wpi.first.wpilibj.GenericHID.RumbleType;
import edu.wpi.first.units.measure.ImmutableAngle;

public class Wrapper {
    public static SparkLowLevel.MotorType kBrushless() {
        return SparkLowLevel.MotorType.kBrushless;
    }
    public static SparkLowLevel.MotorType kBrushed() {
        return SparkLowLevel.MotorType.kBrushed;
    }

    public static SparkBaseConfig.IdleMode kBrake() { return SparkBaseConfig.IdleMode.kBrake; }
    public static SparkBaseConfig.IdleMode kCoast() { return SparkBaseConfig.IdleMode.kCoast; }

    public static SparkBase.ControlType kPosition() { return SparkBase.ControlType.kPosition; }
    public static SparkBase.ControlType kVelocity() { return SparkBase.ControlType.kVelocity; }

    public static int getAllianceStation() {
        AllianceStationID allianceID = DriverStationJNI.getAllianceStation();
        switch (allianceID) {
        case Blue1:
            return 4;
        case Blue2:
            return 5;
        case Blue3:
            return 6;
        case Red1:
            return 1;
        case Red2:
            return 2;
        case Red3:
            return 3;
        default:
            return 0;
        }
    }

    public static void startNetworkTables() {
        NetworkTableInstance inst = NetworkTableInstance.getDefault();
        inst.startServer("/home/lvuser/networktables.json");

        LiveWindow.setEnabled(false);
        Shuffleboard.disableActuatorWidgets();
    }

    public static double test() {
        System.err.println("Test was called!");
        return 2.2;
    }

    public static AHRS createAHRS() {
        return new AHRS(AHRS.NavXComType.kMXP_SPI);
    }

    public static double getAngle(AHRS navx) {
        return navx.getAngle();
    }

    public static RumbleType kBothRumble() {
        return RumbleType.kBothRumble;
    }

    public static RumbleType kLeftRumble() {
        return RumbleType.kLeftRumble;
    }

    public static RumbleType kRightRumble() {
        return RumbleType.kRightRumble;
    }

    public static double getValue(StatusSignal<ImmutableAngle> holder) {
        return holder.getValue().magnitude();
    }

    /*public static void setSpeed(TalonFX motor, double speed) {
      motor.set(ControlMode.PercentOutput, speed);
    }*/

    //public static TalonFXInvertType TalonFXCounterClockwise() { return TalonFXInvertType.CounterClockwise; }

    public static SendableChooser<Integer> createIntegerSendableChooser() {
        return new SendableChooser<Integer>();
    }

    public static SendableChooser<Integer> autoChooser() {
        SendableChooser<Integer> chooser = new SendableChooser<>();
        chooser.addOption("pass on right", 1);
        chooser.setDefaultOption("top", 4);
        chooser.addOption("eco", 3);
        chooser.addOption("test", 2);

        SmartDashboard.putData(chooser);
        return chooser;
    }

    private static final Field2d m_field = new Field2d();

    public static void putField() {
        SmartDashboard.putData("field", m_field);
    }

    public static void setPosition(double x, double y, double theta) {
        m_field.setRobotPose(x, y, Rotation2d.fromRadians(theta));
    }

    public static PneumaticsModuleType revPH() {
        return PneumaticsModuleType.REVPH;
    }

    public static PneumaticsModuleType ctrePCM() {
        return PneumaticsModuleType.CTREPCM;
    }
}
