#include "colors.inc"
#include "shapes.inc"
#include "stones.inc"

camera {
	location <5, 5, -10>
	look_at <0, 0, 0>
	angle 20
}

light_source { <0, 10, -10> color White }

object {
	Sphere
	texture { T_Stone12 }
}
