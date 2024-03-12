
#include "AHRS.h"

namespace navx_wrapper {
    AHRS* bind_navx_mxp() {
        return new AHRS(frc::SPI::Port::kMXP);
    }
}
