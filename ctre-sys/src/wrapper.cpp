#include "bits/predefined_ops.h"
#include "ctre/phoenix6/core/CoreTalonFX.hpp"
#include "ctre/phoenix6/TalonFX.hpp"
#include "ctre/phoenix6/CANcoder.hpp"

int test() {
    return 45;
}

using namespace ctre::phoenix6::hardware;
using namespace ctre::phoenix6;

namespace talonfx_wrapper {
    void set_speed(TalonFX* talon, double speed) {
        talon->SetControl(controls::DutyCycleOut{speed});
    }

    void brake(TalonFX* talon) {
        talon->SetControl(controls::StaticBrake{});
    }

    // rotations per second
    double get_velocity(TalonFX* talon) {
        return talon->GetVelocity().GetValue().value();
    }

    // rotations 
    double get_position(TalonFX* talon) {
        return talon->GetPosition().GetValue().value();
    }

    // rotations 
    void set_position(TalonFX* talon, double position) {
        units::angle::turn_t location{position};
        talon->SetControl(controls::PositionDutyCycle{location});
    }

    // rotations per second
    void set_velocity(TalonFX* talon, double speed) {
        units::angular_velocity::turns_per_second_t velocity{speed};
        talon->SetControl(controls::VelocityDutyCycle{velocity});
    }

    void play_tone(TalonFX* talon, double hertz) {
        units::frequency::hertz_t frequency{hertz};
        talon->SetControl(controls::MusicTone{frequency});
    }

    void invert(TalonFX* talon, bool inverted) {
        talon->SetInverted(inverted);
    }

    void follow(TalonFX* talon, int other, bool reverse) {
        auto follower = controls::Follower{other, reverse};
        talon->SetControl(follower);
    }

    void stop(TalonFX* talon) {
        talon->StopMotor();
    }

    TalonFX* bind_talon(int id) {
        auto talon = new TalonFX(id);
        return talon;
    }

    TalonFX* bind_talon_with_bus(int id, char* bus) {
        auto talon = new TalonFX(id, bus);
        return talon;
    }
}

namespace cancoder_wrapper {
    double get_absolute_position(CANcoder* cancoder) {
        return cancoder->GetAbsolutePosition().GetValue().value();
    }

    double get_position(CANcoder* cancoder) {
        return cancoder->GetPosition().GetValue().value();
    }

    CANcoder* bind_cancoder(int id) {
        auto cancoder = new CANcoder(id);
        return cancoder;
    }

    CANcoder* bind_cancoder_with_bus(int id, char* bus) {
        auto cancoder = new CANcoder(id, bus);
        return cancoder;
    }
}
