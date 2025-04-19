use hyperwood::{Model, Point, Slat, Variant, Vector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct TroughParameters {
    width: isize,
    depth: isize,
    height: isize,
}

impl TroughParameters {
    pub fn new(width: isize, depth: isize, height: isize) -> Self {
        assert!(height >= 3, "Depth must be equal or greater than 3");
        // TODO: validate minimal and maximal sizes

        Self {
            width,
            depth,
            height,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct TroughProperties {
    width: f32,
    depth: f32,
    height: f32,
    volume: f32,
}

pub fn build_model(
    parameters: TroughParameters,
    variant: Variant,
) -> Model<TroughParameters, TroughProperties> {
    let depth = parameters.depth - 1;
    let kurve_stop = 0;

    // TODO: use a lampda
    let d = f32::sin(-std::f32::consts::PI * 1.0 / (parameters.height - kurve_stop) as f32);

    let mut slats = vec![];

    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;
    let mut volume = 0.0;

    // skids
    slats.push(Slat {
        name: "Skid".to_string(),
        layer: 0,
        origin: Point {
            x: -d,
            y: -d,
            z: 0.0,
        },
        vector: Vector {
            x: 0.0,
            y: depth as f32 + 2.0 * d,
            z: 0.0,
        },
    });
    slats.push(Slat {
        name: "Skid".to_string(),
        layer: 0,
        origin: Point {
            x: (parameters.width - 1) as f32 + d,
            y: -d,
            z: 0.0,
        },
        vector: Vector {
            x: 0.0,
            y: depth as f32 + 2.0 * d,
            z: 0.0,
        },
    });

    // floor
    for y in 0..depth + 1 {
        slats.push(Slat {
            name: "Floor".to_string(),
            layer: 1,
            origin: Point {
                x: 0.0,
                y: y as f32,
                z: 1.0,
            },
            vector: Vector {
                x: (parameters.width - 1) as f32,
                y: 0.0,
                z: 0.0,
            },
        });
    }

    // walls
    for z in 2..parameters.height {
        let d = f32::sin(
            std::f32::consts::PI * (z - 1) as f32 / (parameters.height - kurve_stop) as f32,
        );
        let w = parameters.width as f32 + 2.0 * d;
        let h = parameters.height as f32 + d;

        width = width.max(w + 1.0);
        height = height.max(h + 1.0);
        volume += (h - 1.0) * (w - 1.0);

        if z % 2 == 0 {
            // left
            slats.push(Slat {
                name: "Wall side even".to_string(),
                layer: z,
                origin: Point {
                    x: -d,
                    y: -d,
                    z: z as f32,
                },
                vector: Vector {
                    x: 0.0,
                    y: depth as f32 + 2.0 * d,
                    z: 0.0,
                },
            });

            // right
            slats.push(Slat {
                name: "Wall side even".to_string(),
                layer: z,
                origin: Point {
                    x: (parameters.width - 1) as f32 + d,
                    y: -d,
                    z: z as f32,
                },
                vector: Vector {
                    x: 0.0,
                    y: depth as f32 + 2.0 * d,
                    z: 0.0,
                },
            });

            // front
            slats.push(Slat {
                name: "Wall even".to_string(),
                layer: z,
                origin: Point {
                    x: 1.0 - d,
                    y: -d,
                    z: z as f32,
                },
                vector: Vector {
                    x: (parameters.width - 1) as f32 - 2.0 + 2.0 * d,
                    y: 0.0,
                    z: 0.0,
                },
            });

            // back
            slats.push(Slat {
                name: "Wall even".to_string(),
                layer: z,
                origin: Point {
                    x: 1.0 - d,
                    y: depth as f32 + d,
                    z: z as f32,
                },
                vector: Vector {
                    x: (parameters.width - 1) as f32 - 2.0 + 2.0 * d,
                    y: 0.0,
                    z: 0.0,
                },
            });
        } else {
            // left
            slats.push(Slat {
                name: "Wall side odd".to_string(),
                layer: z,
                origin: Point {
                    x: -d,
                    y: 1.0 - d,
                    z: z as f32,
                },
                vector: Vector {
                    x: 0.0,
                    y: depth as f32 - 2.0 + 2.0 * d,
                    z: 0.0,
                },
            });

            // right
            slats.push(Slat {
                name: "Wall side odd".to_string(),
                layer: z,
                origin: Point {
                    x: (parameters.width - 1) as f32 + d,
                    y: 1.0 - d,
                    z: z as f32,
                },
                vector: Vector {
                    x: 0.0,
                    y: depth as f32 - 2.0 + 2.0 * d,
                    z: 0.0,
                },
            });

            // front
            slats.push(Slat {
                name: "Wall odd".to_string(),
                layer: z,
                origin: Point {
                    x: -d,
                    y: -d,
                    z: z as f32,
                },
                vector: Vector {
                    x: (parameters.width - 1) as f32 + 2.0 * d,
                    y: 0.0,
                    z: 0.0,
                },
            });

            // back
            slats.push(Slat {
                name: "Wall odd".to_string(),
                layer: z,
                origin: Point {
                    x: -d,
                    y: depth as f32 + d,
                    z: z as f32,
                },
                vector: Vector {
                    x: (parameters.width - 1) as f32 + 2.0 * d,
                    y: 0.0,
                    z: 0.0,
                },
            });
        }
    }

    let properties = TroughProperties {
        width: width * variant.x,
        depth: parameters.depth as f32 * variant.y,
        height: height * variant.z,
        volume: volume * variant.x * variant.y * variant.z,
    };

    Model {
        parameters,
        properties,
        name: "Trough".to_owned(),
        variant,
        slats,
    }
}
