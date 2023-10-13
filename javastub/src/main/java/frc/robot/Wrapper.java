package frc.robot;

import com.revrobotics.CANSparkMaxLowLevel.MotorType;

public class Wrapper {
    public static MotorType kBrushless() {
        return MotorType.kBrushless;
    }
    public static MotorType kBrushed() {
        return MotorType.kBrushed;
    }

    public static String test() { return "hello"; }
}
