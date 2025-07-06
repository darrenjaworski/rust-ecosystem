# ecosystem in a bottle CLI game

A rust cli game that models an enclosed ecosystem. A player is seeking balance for their life in a bottle.

## variables

The following relationships are modeled using ODE.

### biotic actors

- plants (in the soil and extending into air)
- microbes (in the soil)
- worms (in the soil)
- shrimp (in the water)

plants do photosynthesis at a rate
plants consume co2
plants consume nitrogen from the soil
plants produce oxygen

microbes do nitrogen fixation at a rate
microbes grow or die at a rate
microbes convert n2 gas into nitrogen in the soil
microbes consume oxygen

## ODEs for ecosystem relationships

**Legend:**

- P: plant biomass
- M: microbe population
- W: worm population
- S: shrimp population
- D: detritus
- N_soil: soil nitrogen
- O2: oxygen
- CO2: carbon dioxide
- N2: nitrogen gas
- Wtr: water volume
- T: temperature
- H: humidity
- pH: soil pH
- R: rocks

### Plant Photosynthesis and Respiration

**Day (Photosynthesis):**

- dO2/dt = k_photo _ P _ L _ f_hum(H) _ f_CO2(CO2)
- dCO2/dt = -alpha_photo \* dO2/dt
- dN_soil/dt = -k_N_plant \* P
- dP/dt = k_grow _ P _ f_light(L) _ f_nutr(N_soil) _ f_hum(H) \* f_comp(P)

**Night (Respiration):**

- dO2/dt = -k_resp \* P
- dCO2/dt = -alpha_resp \* dO2/dt

### Microbial Nitrogen Fixation and Respiration

- dN_soil/dt = k_fix _ M _ f_O2(O2) \* f_moist(Wtr)
- dM/dt = k_M_grow _ M _ f_N(N_soil) _ f_moist(Wtr) _ f_temp(T) - k_M_death _ M _ f_pH(pH) \* f_O2(O2)
- dO2/dt = -k_M_resp \* M
- dCO2/dt = -alpha_M_resp \* dO2/dt

### Worms

- dA_soil/dt = k_worm_air \* W
- dD/dt = -k_worm_decomp \* W
- dW/dt = k_W_grow _ W _ f_det(D) _ f_moist(Wtr) - k_W_death _ W \* f_tox(tox)

### Shrimp

- dD/dt = -k_shrimp_det \* S
- dN_soil/dt += k_shrimp_waste \* S
- dS/dt = k_S_grow _ S _ f_det(D) _ f_O2(O2) - k_S_death _ S \* f_tox(tox)

### Soil, Water, and Air

- dpH/dt = -k_acid _ M + k_buffer_rock _ R + k_buffer_water \* Wtr
- dWtr/dt = input - evaporation - uptake
- dN2/dt = -k_fix \* M
- dN_soil/dt += k_fix \* M
- dO2/dt = (sum of all O2 sources/sinks)
- dCO2/dt = (sum of all CO2 sources/sinks)

### Relationships/Modifiers

- f_light(L): increases with light, saturates at high L
- f_hum(H): increases with humidity, plateaus
- f_temp(T): bell curve, optimal at some T
- f_nutr(N_soil): saturating function
- f_comp(P) = 1 - (P/P_max): competition
- f_moist(Wtr): optimal range for water
- f_pH(pH): optimal range for pH
- f_O2(O2): low O2 penalizes growth
- f_det(D): more detritus, more food for worms/shrimp
- f_tox(tox): high toxicity increases death

### Collapse/Failure

If any population (P, M, W, S) falls below a threshold, trigger destabilization or collapse events.

## relationships

The following relationships are modeled using ODE.

### biotic actors

- plants (in the soil and extending into air)
- microbes (in the soil)
- worms (in the soil)
- shrimp (in the water)

plants do photosynthesis at a rate
plants consume co2
plants consume nitrogen from the soil
plants produce oxygen

microbes do nitrogen fixation at a rate
microbes grow or die at a rate
microbes convert n2 gas into nitrogen in the soil
microbes consume oxygen

relationships

- higher light levels increase the rate of photosynthesis
- the faster the rate of photosynthesis the faster the plants grow
- higher humidity increases plant growth rate (up to a point)
- higher temperature increases both plant and microbial growth (but too high can be harmful)
- more water increases humidity and supports plant/microbe growth, but too much can cause root rot or oxygen depletion
- more soil increases microbial, worm, and plant capacity
- porous soil increases microbial and worm growth rate and water drainage
- nonporous soil slows microbial and worm growth and retains more water
- more plants increase total oxygen production, but also increase competition for resources (light, water, nutrients)
- larger plants increase competition, slowing further growth
- microbes consume oxygen and produce CO2, but also fix nitrogen (improving soil fertility)
- microbes convert atmospheric N2 into soil nitrogen, supporting plant growth
- plants consume CO2 and produce O2 during the day (photosynthesis)
- plants consume O2 and produce CO2 at night (respiration)
- pH is lowered by microbial activity and raised by water buffering; extreme pH harms both plants and microbes
- air composition (N2, O2, CO2) is affected by plant, microbe, worm, and shrimp activity and must remain in a habitable range
- worms aerate the soil, improving plant and microbe growth, and help decompose organic matter
- shrimp consume detritus in the water, help recycle nutrients, and produce waste that can fertilize plants
- shrimp and worms both contribute to the breakdown of dead organic matter, closing nutrient loops
- rocks can buffer pH and provide microhabitats for microbes and worms
- if any population (plants, microbes, worms, shrimp) collapses, it can destabilize the whole system
