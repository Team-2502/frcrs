#include "ctre/phoenix6/core/CoreTalonFX.hpp"
#include "ctre/phoenix6/TalonFX.hpp"

int test() {
    return 45;
}

using namespace ctre::phoenix6::hardware;
using namespace ctre::phoenix6;

TalonFX* CreateTalonFX(int id) {
    auto talon = new TalonFX(id);
    return talon;
};

void SetSpeed(TalonFX* talon, double speed) {
    talon->SetControl(controls::DutyCycleOut{speed});
};
