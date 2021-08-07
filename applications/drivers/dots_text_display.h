#pragma once

#include "tock.h"

bool display_is_present (void);
bool display_set_speed (unsigned int ms);
bool display_get_speed (unsigned int *ms);