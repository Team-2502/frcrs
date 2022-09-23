package frc.robot;
import com.revrobotics.CANSparkMaxLowLevel.MotorType;

/**
 * support class for rust bindings
 * re-exports nested enums, which JNI struggles with
 * */
public class Reexports {
    // sparkmax
    public static MotorType[] motorTypes = MotorType.values();
    public static MotorType kBrushless = MotorType.kBrushless;
}
