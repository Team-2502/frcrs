package frc.robot;

import com.revrobotics.CANSparkMaxLowLevel.MotorType;
import com.revrobotics.CANSparkMax.IdleMode;
import com.revrobotics.CANSparkMax.ControlType;

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

    //public static TalonFXInvertType TalonFXCounterClockwise() { return TalonFXInvertType.CounterClockwise; }
}
