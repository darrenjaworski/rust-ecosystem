# ecosystem in a bottle CLI game

A rust cli game that models an enclosed ecosystem. A player is seeking balance for their life in a bottle. There is a day and night cycle. Day has 10 intervals, night has 6 intervals.

## game setup

- a player selects the type of soil they want to choose

  - porous
  - non porous

- a player selects the number of plants they want

  - a number between 2-5

- a player selects how much soil they put into the bottle

  - a number between 10 and 30 kg

- a player chooses how close they want the bottle to be to the window
- a player chooses how much water there is in the bottle
- how many rocks are put in the bottle
  - a number between 2-5

## how these variables relate

- porous soil allow for faster microbial growth
- nonporous soil slow the rate of microbial growth
- each plant generates oxygen from co2
- how much soil determines how much microbial growth occurs
  - more soil means more overall capacity for microbes
- closeness to the window determines the growth rate of the plants
- closeness to the window determines the bottle temperature
- the higher the temperature and the more water, the higher the humidity
- the higher the temperature the faster the microbial growth
- the higher the humidity the faster the plant growth
- the larger the plants the more competition between them, so the slower growth for some than others
- sitting water leads to increased microbe growth
- microbes in the water produce oxygen
- during the day, the plans thrive and grow

### adjustable ongoing variables

- ph
- light exposure
- humidity

With each generation a player is shown the following statistics:

- ph (a value between 0-14)
- air composition
  - percent adds up to 100
    - nitrogen
    - co2
    - oxygen
- a hygrometer showing the humidity (a value from 0-100)
- plant size (a value between 1-100)
- microbial levels (a value between 1-100)
