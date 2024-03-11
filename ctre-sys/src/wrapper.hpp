#include "ctre/phoenix6/core/CoreTalonFX.hpp"
#include "ctre/phoenix6/TalonFX.hpp"

int test() {
    return 45;
}


ctre::phoenix6::hardware::TalonFX* CreateTalonFX(int id) {
    auto talon = new ctre::phoenix6::hardware::TalonFX(id);
    return talon;
};
