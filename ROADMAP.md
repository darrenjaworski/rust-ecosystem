# ecosystem in a bottle CLI game

A rust cli game that models an enclosed ecosystem. A player is seeking balance for their life in a bottle.

## variables

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
