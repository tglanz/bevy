use crate::render::{draw::Draw, Camera};
use crate::core::float_ord::FloatOrd;
use crate::transform::prelude::Transform;
use legion::prelude::*;

#[derive(Debug)]
pub struct VisibleEntity {
    pub entity: Entity,
    pub order: FloatOrd,
}

#[derive(Default, Debug)]
pub struct VisibleEntities {
    pub value: Vec<VisibleEntity>,
}

impl VisibleEntities {
    pub fn iter(&self) -> impl DoubleEndedIterator<Item=&VisibleEntity> {
        self.value.iter()
    }
}

pub fn visible_entities_system(
    world: &mut SubWorld,
    camera_query: &mut Query<(Read<Camera>, Write<VisibleEntities>)>,
    entities_query: &mut Query<Read<Draw>>,
    _transform_query: &mut Query<Read<Transform>>,
    _transform_entities_query: &mut Query<(Read<Draw>, Read<Transform>)>, // ensures we can optionally access Transforms
) {
    let (mut camera_world, world) = world.split_for_query(camera_query);
    for (camera_entity, (_camera, mut visible_entities)) in camera_query.iter_entities_mut(&mut camera_world) {
        visible_entities.value.clear();
        let camera_transform = world.get_component::<Transform>(camera_entity).unwrap();
        let camera_position = camera_transform.value.w_axis().truncate();

        let mut no_transform_order = 0.0;
        let mut transparent_entities = Vec::new();
        for (entity, draw) in entities_query.iter_entities(&world) {
            if !draw.is_visible {
                continue;
            }

            let order = if let Some(transform) = world.get_component::<Transform>(entity) {
                let position = transform.value.w_axis().truncate();
                // smaller distances are sorted to lower indices by using the distance from the camera 
                FloatOrd((camera_position - position).length())
            } else {
                let order = FloatOrd(no_transform_order);
                no_transform_order += 0.1;
                order
            };

            if draw.is_transparent {
                transparent_entities.push(VisibleEntity {
                    entity,
                    order,
                })
            } else {
                visible_entities.value.push(VisibleEntity {
                    entity,
                    order,
                })
            }
        }


        // sort opaque entities front-to-back
        visible_entities.value.sort_by_key(|e| e.order);

        // sort transparent entities front-to-back
        transparent_entities.sort_by_key(|e|-e.order);
        visible_entities.value.extend(transparent_entities);

        // TODO: check for big changes in visible entities len() vs capacity() (ex: 2x) and resize to prevent holding unneeded memory
    }
}