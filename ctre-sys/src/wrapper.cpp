#include "ctre/phoenix6/core/CoreTalonFX.hpp"
#include "ctre/phoenix6/TalonFX.hpp"

int test() {
    return 45;
}

using namespace ctre::phoenix6::hardware;
using namespace ctre::phoenix6;

namespace talonfx_wrapper {
    void set_speed(TalonFX* talon, double speed) {
        talon->SetControl(controls::DutyCycleOut{speed});
    }

    void set_velocity(TalonFX* talon, double speed) {
        units::angular_velocity::turns_per_second_t velocity{speed};
        talon->SetControl(controls::VelocityDutyCycle{velocity});
    }

    void play_tone(TalonFX* talon, double hertz) {
        units::frequency::hertz_t frequency{hertz};
        talon->SetControl(controls::MusicTone{frequency});
    }

    TalonFX* bind_talon(int id) {
        auto talon = new TalonFX(id);
        return talon;
    }

    TalonFX* bind_talon_with_bus(int id, char* bus) {
        std::string bus_str = bus;
        auto talon = new TalonFX(id, bus);
        return talon;
    }
}
