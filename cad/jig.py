import cadquery as cq

led_rad = 2.5
led_spacing = 20
edge_spacing = 10
jig_height = 8
hole_depth = 6
width = 3
length = 3


def go():
    wp = cq.Workplane("XY")

    b = (
        wp.box(
            width=edge_spacing * 2 + led_spacing * (width - 1),
            length=edge_spacing * 2 + led_spacing * (length - 1),
            height=jig_height,
        )
        .edges("|Z")
        .fillet(1)
        .faces("+Z")
        .fillet(1)
    )

    result = (
        b.faces(">Z")
        .workplane()
        .rarray(led_spacing, led_spacing, width, length)
        .hole(led_rad * 2, hole_depth)
    )

    return result


result = go()

if __name__ == '__main__':
    cq.exporters.export(result, "jig.stl", None, 0.01, 0.1)
