package frc.robot;

import com.revrobotics.CANSparkMaxLowLevel.MotorType;

public class Wrapper {
    public static MotorType getBrushless() {
        return MotorType.kBrushless;
    }

    public static String test() { return "hello"; }
}