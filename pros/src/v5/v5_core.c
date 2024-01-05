#include "v5_api.h"

uint32_t comp_state = V5_COMP_BIT_MODE | V5_COMP_BIT_COMP; // disabled, auton, connected, field control

// comp status
uint32_t vexCompetitionStatus() { return comp_state; }