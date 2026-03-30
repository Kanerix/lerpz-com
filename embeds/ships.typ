#set document(
  title: "Ships: An Overview of Maritime Vessels",
  author: "Lerpz Example Documents",
  keywords: ("ships", "maritime", "vessels", "navigation", "ocean"),
)

#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 3cm),
)

#set text(font: "New Computer Modern", size: 11pt)
#set heading(numbering: "1.1.")
#set par(justify: true, leading: 0.65em)

#align(center)[
  #text(size: 22pt, weight: "bold")[Ships: An Overview of Maritime Vessels]
  #v(0.5em)
  #text(size: 13pt, style: "italic")[A reference document for RAG embedding tests]
  #v(0.3em)
  #text(size: 10pt, fill: gray)[Lerpz Example Documents]
]

#v(1em)
#line(length: 100%, stroke: 0.5pt + gray)
#v(1em)

= Introduction

Ships are large watercraft that travel across oceans, seas, lakes, and rivers, serving as one of the oldest and most important forms of human transportation. Since the earliest civilizations, humans have built vessels to explore new lands, trade goods, wage war, and harvest the sea's abundant resources. Today, the global shipping industry moves more than 80% of all international trade by volume, making maritime transport the backbone of the world economy.

The history of shipbuilding spans thousands of years, from simple reed boats used in ancient Mesopotamia and Egypt to the sophisticated nuclear-powered aircraft carriers and container mega-ships of the modern era. Each era brought new materials, propulsion methods, and hull designs that expanded what was possible at sea.

= Types of Ships

== Cargo Vessels

Cargo ships form the largest category of commercial vessels. They are designed to carry goods efficiently across long distances with minimal crew.

- *Container ships* carry standardized intermodal containers stacked on deck and in holds. The largest container ships, known as ultra-large container vessels (ULCVs), can carry more than 24,000 twenty-foot equivalent units (TEUs).
- *Bulk carriers* transport unpackaged dry commodities such as coal, grain, iron ore, and fertilizer. Their cavernous holds and deck cranes allow them to load and unload large quantities quickly.
- *Tankers* carry liquid cargo, most commonly crude oil, refined petroleum products, liquefied natural gas (LNG), or chemicals. Double-hull construction is now mandatory to reduce the risk of spills.
- *Roll-on/roll-off (RoRo) ships* are designed so that wheeled cargo—cars, trucks, and heavy machinery—can be driven directly on and off through large stern or side ramps.

== Passenger Vessels

Passenger ships range from small river ferries to enormous ocean-going cruise liners. Modern cruise ships are essentially floating resort cities, offering thousands of cabins alongside restaurants, theatres, swimming pools, and shopping centres. The largest cruise ships in service today displace more than 200,000 gross tons and carry over 6,000 passengers.

Ferries provide essential short-sea transportation links between islands and across straits where building bridges is impractical. High-speed ferries use catamaran or hydrofoil designs to achieve speeds exceeding 40 knots.

== Naval Vessels

Warships are purpose-built for combat or supporting military operations at sea.

- *Aircraft carriers* serve as floating air bases, projecting air power far from home shores. Nuclear-powered carriers operated by the United States Navy can remain at sea for years without refuelling.
- *Destroyers and frigates* are fast, manoeuvrable surface combatants capable of anti-submarine warfare, anti-air defence, and surface warfare.
- *Submarines* operate beneath the surface, providing stealth for surveillance, deterrence, and strike missions. Ballistic missile submarines carry nuclear warheads and serve as a key component of national deterrence strategies.
- *Amphibious assault ships* transport marines and their equipment to hostile shores, combining the functions of a carrier, landing craft, and troop transport.

== Specialised Vessels

Many ships are designed for highly specific tasks:

- *Icebreakers* use reinforced hulls and powerful engines to break through sea ice, keeping Arctic and Antarctic supply routes open.
- *Cable-laying ships* deploy and repair undersea telecommunications and power cables.
- *Dredgers* remove sediment from harbour floors and shipping channels to maintain navigable depths.
- *Offshore supply vessels (OSVs)* support oil and gas platforms with equipment, fuel, and crew transfers.
- *Research vessels* are equipped with laboratories and specialised instruments for oceanographic, geological, and biological studies.

= Ship Anatomy and Structure

A ship's hull is its watertight body, shaped to move efficiently through water. The hull is characterised by the *bow* (front), *stern* (rear), *port* (left side) and *starboard* (right side). The *keel* runs along the centreline of the bottom and is the structural backbone of the ship.

The *superstructure* rises above the main deck and houses the bridge—from where the ship is navigated—as well as crew quarters, engine control rooms, and cargo handling equipment. Modern bridges are packed with electronic navigation systems, including GPS, radar, electronic chart display systems (ECDIS), and automatic identification systems (AIS).

Below the main deck, the ship is divided into watertight compartments. This subdivision limits flooding in the event of hull damage, improving survivability. The engine room, typically located aft, houses the main propulsion machinery, generators, and auxiliary systems.

= Propulsion Systems

== Diesel Engines

The vast majority of commercial ships are powered by large two-stroke marine diesel engines. These slow-running engines are extraordinarily fuel-efficient and can burn cheaper, heavy fuel oil. A single engine may develop tens of thousands of kilowatts and weigh thousands of tonnes.

== Gas Turbines

Gas turbines offer a high power-to-weight ratio and are favoured in naval applications where speed and responsiveness are critical. They can also be combined with diesel engines in combined diesel and gas (CODAG) arrangements.

== Nuclear Propulsion

Nuclear reactors heat steam to drive turbines, providing virtually unlimited range. This technology is used in some naval vessels, particularly aircraft carriers and submarines, but high cost and regulatory complexity have prevented its widespread adoption in merchant shipping.

== LNG and Alternative Fuels

Liquefied natural gas (LNG) has emerged as a cleaner-burning alternative to conventional heavy fuel oil, producing significantly lower sulphur dioxide and particulate emissions. The shipping industry is also exploring hydrogen, ammonia, methanol, and wind-assisted propulsion as pathways to decarbonisation in line with International Maritime Organization (IMO) greenhouse gas targets.

= Navigation and Safety

Modern navigation relies on an integrated suite of electronic systems. The Global Navigation Satellite System (GNSS), which includes GPS and Galileo, provides precise position fixes anywhere on Earth. Radar systems detect other vessels and obstacles in poor visibility, while ECDIS displays real-time charts with the ship's position overlaid.

The International Maritime Organization (IMO) sets global standards for ship safety, crew training, and environmental protection. The International Convention for the Safety of Life at Sea (SOLAS) mandates safety equipment, fire protection systems, lifeboats, and emergency procedures. The Standards of Training, Certification and Watchkeeping (STCW) convention defines the minimum competencies required of seafarers worldwide.

= Environmental Impact and Sustainability

Shipping is one of the more carbon-efficient forms of freight transport per tonne-kilometre, but the sheer scale of global trade means the industry contributes roughly 3% of global greenhouse gas emissions. SOx and NOx emissions from burning heavy fuel oil also contribute to air pollution, particularly near port cities.

The IMO has introduced the Energy Efficiency Design Index (EEDI) for new ships and the Carbon Intensity Indicator (CII) to drive fleet-wide efficiency improvements. Slow steaming—reducing vessel speed to cut fuel consumption—has become standard practice. Innovations such as air lubrication systems, hull coatings that reduce biofouling, and wind-assisted propulsion devices like rigid sails and rotor sails are being adopted to reduce fuel burn and emissions.

= Historical Milestones

#table(
  columns: (auto, 1fr),
  stroke: 0.5pt + gray,
  fill: (_, row) => if calc.odd(row) { luma(240) } else { white },
  table.header([*Period*], [*Milestone*]),
  [~3000 BCE], [Ancient Egyptians and Mesopotamians build oared and sailed wooden vessels for river and coastal trade.],
  [~1000 CE], [Viking longships enable exploration of the North Atlantic, reaching Iceland, Greenland, and North America.],
  [1492], [Columbus crosses the Atlantic using square-rigged caravel vessels, opening sustained contact between Europe and the Americas.],
  [1807], [Robert Fulton's _Clermont_ demonstrates practical steamship propulsion on the Hudson River.],
  [1845], [SS _Great Britain_, designed by Isambard Kingdom Brunel, becomes the first ocean-going ship with an iron hull and screw propeller.],
  [1912], [RMS _Titanic_ sinks after striking an iceberg, leading to major reforms in maritime safety regulations.],
  [1956], [Malcolm McLean introduces the standardised shipping container, revolutionising global trade logistics.],
  [1959], [USS _George Washington_ becomes the first ballistic missile submarine to conduct a submerged launch.],
  [2006], [Emma Maersk launches as the world's largest container ship at the time, heralding the era of mega-vessels.],
  [2023], [The first large commercial vessels powered by green methanol enter service, marking a step toward zero-emission shipping.],
)

= Conclusion

Ships have shaped civilisation more profoundly than almost any other technology. They have enabled trade, exploration, cultural exchange, and conflict across every era of recorded history. Today, with the dual pressures of maintaining global trade flows and sharply reducing carbon emissions, the shipping industry faces one of its greatest engineering challenges. New hull forms, alternative fuels, digital automation, and autonomous vessel technology promise to define the next chapter in the long story of maritime transport.

The enormous diversity of vessel types—from tiny harbour tugs to nuclear-powered supercarriers—reflects the equally diverse demands humanity places on the sea. Understanding ships means understanding much of how the modern world works.
