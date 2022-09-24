package frc.robot;
import com.revrobotics.CANSparkMaxLowLevel.MotorType;
import com.revrobotics.CANSparkMax;

/**
 * support class for rust bindings
 * re-exports nested enums, which JNI struggles with
 * */
public class Reexports {
    // sparkmax
    public static final MotorType[] motorTypes = MotorType.values();
    public static final MotorType kBrushless = MotorType.kBrushless;

    public static final CANSparkMax.IdleMode[] idleModes= CANSparkMax.IdleMode.values();
    public static final CANSparkMax.ExternalFollower phoenixFollower = CANSparkMax.ExternalFollower.kFollowerPhoenix;
    public static final CANSparkMax.ExternalFollower sparkFollower = CANSparkMax.ExternalFollower.kFollowerSparkMax;
}
