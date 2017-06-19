use std::collections::BTreeMap;
use nalgebra::{Matrix4, Vector3, UnitQuaternion, U1, U3, Rotation};
use world::entity::Entity;

pub type Transform = u32;

type Quaternion<F> = UnitQuaternion<F>;

pub struct SceneManager {
    transforms: BTreeMap<Entity, Transform>,

    // Component data buffers.
    entity: Vec<Entity>,
    local: Vec<Matrix4<f32>>,
    world: Vec<Matrix4<f32>>,
    parent: Vec<Transform>,
    child: Vec<Transform>,
    last_sibling: Vec<Transform>,
    next_sibling: Vec<Transform>,
    dirty: Vec<bool>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            transforms: BTreeMap::new(),
            entity: Vec::new(),
            local: Vec::new(),
            world: Vec::new(),
            parent: Vec::new(),
            child: Vec::new(),
            last_sibling: Vec::new(),
            next_sibling: Vec::new(),
            dirty: Vec::new(),
        }
    }

    pub fn create_transform(&mut self, entity: Entity) -> Transform {
        let next = self.entity.len() as u32;

        self.entity.push(entity);
        self.local.push(Matrix4::identity());
        self.world.push(Matrix4::identity());
        self.parent.push(Transform::max_value());
        self.child.push(Transform::max_value());
        self.last_sibling.push(Transform::max_value());
        self.next_sibling.push(Transform::max_value());
        self.dirty.push(false);

        // FIXME: Use returned value to guard against orphaned transforms?
        self.transforms.insert(entity, next);

        next
    }

    pub fn destroy_transform(&mut self, transform: Transform) {
        assert!(transform < self.entity.len() as u32, "transform {} does not exist");

        let target = transform as usize;
        let last = self.entity.len() - 1 as usize;

        let entity = self.entity[target];
        let last_entity = self.entity[last];

        self.entity.swap_remove(target);
        self.local.swap_remove(target);
        self.world.swap_remove(target);
        self.parent.swap_remove(target);
        self.child.swap_remove(target);
        self.last_sibling.swap_remove(target);
        self.next_sibling.swap_remove(target);
        self.dirty.swap_remove(target);

        self.transforms.insert(last_entity, transform);
        self.transforms.remove(&entity);
    }

    pub fn has_transform(&self, entity: Entity) -> bool {
        self.transforms.contains_key(&entity)
    }

    pub fn transform_for(&self, entity: Entity) -> Transform {
        match self.transforms.get(&entity) {
            Some(transform) => *transform,
            None => panic!("entity {} has no transform")
        }
    }

    pub fn set_local_position(&mut self, entity: Entity, position: Vector3<f32>) {
        let instance = self.transform_for(entity);

        // TODO: Work out the actual borrowing problem here.
        {
            let local = &mut self.local[instance as usize];
            let mut slice = local.slice_mut((3, 0), (1, 3));
            slice[0] = position[0];
            slice[1] = position[1];
            slice[2] = position[2];
        }

        self.apply(instance);
    }

    pub fn local_position(&self, entity: Entity) -> Vector3<f32> {
        let instance = self.transform_for(entity) as usize;

        let slice = self.local[instance].slice((3, 0), (1, 3));

        Vector3::new(slice[0], slice[1], slice[2])
    }

    pub fn set_local_rotation(&mut self, entity: Entity, rotation: Quaternion<f32>) {
        let instance = self.transform_for(entity);

        // TODO: Work out the actual borrowing problem here.
        {
            let local = &mut self.local[instance as usize];
            let mut slice = local.fixed_slice_mut::<U3, U3>(0, 0);

            let rot_mat = rotation.to_rotation_matrix();
            let rot_slice = rot_mat.matrix().as_slice();

            for a in 0..9 {
                slice[a] = rot_slice[a];
            }
        }

        self.apply(instance);
    }

    pub fn local_rotation(&self, entity: Entity) -> Quaternion<f32> {
        let instance = self.transform_for(entity) as usize;

        let slice = self.local[instance].fixed_slice::<U3, U3>(0, 0);
        Quaternion::from_rotation_matrix(&Rotation::from_matrix_unchecked(slice.clone_owned()))
    }

    pub fn set_local_scale(&mut self, entity: Entity, scale: Vector3<f32>) {
        let instance = self.transform_for(entity);
        let curr_scale = self.local_scale(entity);

        {
            let local = &mut self.local[instance as usize];
            let mut embed = local.fixed_slice_mut::<U3, U3>(0, 0);

            embed[(0, 0)] = (embed[(0, 0)] / curr_scale[0]) * scale[0];
            embed[(1, 0)] = (embed[(1, 0)] / curr_scale[0]) * scale[0];
            embed[(2, 0)] = (embed[(2, 0)] / curr_scale[0]) * scale[0];

            embed[(0, 1)] = (embed[(0, 1)] / curr_scale[1]) * scale[1];
            embed[(1, 1)] = (embed[(1, 1)] / curr_scale[1]) * scale[1];
            embed[(2, 1)] = (embed[(2, 1)] / curr_scale[1]) * scale[1];

            embed[(0, 2)] = (embed[(0, 2)] / curr_scale[2]) * scale[2];
            embed[(1, 2)] = (embed[(1, 2)] / curr_scale[2]) * scale[2];
            embed[(2, 2)] = (embed[(2, 2)] / curr_scale[2]) * scale[2];
        }

        self.apply(instance);
    }

    pub fn local_scale(&self, entity: Entity) -> Vector3<f32> {
        let instance = self.transform_for(entity) as usize;

        let local = self.local[instance];
        let mut embed = local.fixed_slice::<U3, U3>(0, 0);

        let scale_x = (embed[(0, 0)].powi(2) + embed[(1, 0)].powi(2) + embed[(2, 0)].powi(2)).sqrt();
        let scale_y = (embed[(0, 1)].powi(2) + embed[(1, 1)].powi(2) + embed[(2, 1)].powi(2)).sqrt();
        let scale_z = (embed[(0, 2)].powi(2) + embed[(1, 2)].powi(2) + embed[(2, 2)].powi(2)).sqrt();

        Vector3::new(scale_x, scale_y, scale_z)
    }

    pub fn set_world_position(&mut self, entity: Entity, position: Vector3<f32>) {
        let instance = self.transform_for(entity);

        // TODO: Work out the actual borrowing problem here.
        {
            let world = &mut self.world[instance as usize];
            let mut slice = world.slice_mut((3, 0), (1, 3));
            slice[0] = position[0];
            slice[1] = position[1];
            slice[2] = position[2];
        }
    }

    pub fn world_position(&self, entity: Entity) -> Vector3<f32> {
        let instance = self.transform_for(entity) as usize;

        let slice = self.world[instance].slice((3, 0), (1, 3));

        Vector3::new(slice[0], slice[1], slice[2])
    }

    pub fn set_world_rotation(&mut self, entity: Entity, rotation: Quaternion<f32>) {
        let instance = self.transform_for(entity);

        // TODO: Work out the actual borrowing problem here.
        {
            let world = &mut self.world[instance as usize];
            let mut slice = world.fixed_slice_mut::<U3, U3>(0, 0);

            let rot_mat = rotation.to_rotation_matrix();
            let rot_slice = rot_mat.matrix().as_slice();

            for a in 0..9 {
                slice[a] = rot_slice[a];
            }
        }
    }

    pub fn world_rotation(&self, entity: Entity) -> Quaternion<f32> {
        let instance = self.transform_for(entity) as usize;

        let slice = self.world[instance].fixed_slice::<U3, U3>(0, 0);
        Quaternion::from_rotation_matrix(&Rotation::from_matrix_unchecked(slice.clone_owned()))
    }

    pub fn link(&mut self, child_ent: Entity, parent_ent: Entity) {
        let child = self.transform_for(child_ent);
        let parent = self.transform_for(parent_ent);

        self.unlink(child_ent);

        if self.child[parent as usize] == Transform::max_value() {
            self.child[parent as usize] = child;
            self.parent[child as usize] = parent;
        } else {
            let mut previous = Transform::max_value();
            let mut current = self.child[parent as usize];

            while current != Transform::max_value() {
                previous = current;
                current = self.next_sibling[current as usize];
            }

            self.next_sibling[previous as usize] = child;

            self.child[child as usize] = Transform::max_value();
            self.next_sibling[child as usize] = Transform::max_value();
            self.last_sibling[child as usize] = previous;
        }

        let mut parent_world = self.world[parent as usize];
        let mut child_world = self.world[child as usize];

        let child_scale = Vector3::new(child_world.fixed_slice::<U3, U1>(0, 0).norm(),
                                       child_world.fixed_slice::<U3, U1>(0, 1).norm(),
                                       child_world.fixed_slice::<U3, U1>(0, 2).norm());

        // Normalize the child transform.
        child_world.fixed_slice_mut::<U3, U1>(0, 0).normalize_mut();
        child_world.fixed_slice_mut::<U3, U1>(0, 1).normalize_mut();
        child_world.fixed_slice_mut::<U3, U1>(0, 2).normalize_mut();

        // Normalize the parent transform.
        parent_world.fixed_slice_mut::<U3, U1>(0, 0).normalize_mut();
        parent_world.fixed_slice_mut::<U3, U1>(0, 1).normalize_mut();
        parent_world.fixed_slice_mut::<U3, U1>(0, 2).normalize_mut();

        // Capture the relative transform from parent space to child space.
        let inverted = parent_world.try_inverse().unwrap();
        let mut relative = child_world * inverted;

        relative.fixed_slice_mut::<U3, U1>(0, 0).component_mul_mut(&Vector3::from_element(child_scale[0]));
        relative.fixed_slice_mut::<U3, U1>(0, 1).component_mul_mut(&Vector3::from_element(child_scale[1]));
        relative.fixed_slice_mut::<U3, U1>(0, 2).component_mul_mut(&Vector3::from_element(child_scale[2]));

        self.local[child as usize] = relative;
        self.parent[child as usize] = parent;

        self.transform(child, parent_world);
    }

    pub fn unlink(&mut self, entity: Entity) {
        let instance = self.transform_for(entity) as usize;

        if self.parent[instance] == Transform::max_value() {
            return
        }

        if self.last_sibling[instance] == Transform::max_value() {
            self.child[self.parent[instance] as usize] = self.next_sibling[instance]
        } else {
            self.next_sibling[self.last_sibling[instance] as usize] = self.next_sibling[instance]
        }

        if self.next_sibling[instance] != Transform::max_value() {
            self.last_sibling[self.next_sibling[instance] as usize] = self.last_sibling[instance]
        }

        self.parent[instance] = Transform::max_value();
        self.next_sibling[instance] = Transform::max_value();
        self.last_sibling[instance] = Transform::max_value();
    }

    /// Applies parent transform to local to renormalize.
    pub fn apply(&mut self, transform: Transform) {
        let parent = self.parent[transform as usize];
        let world = if parent == Transform::max_value() {
            Matrix4::identity()
        } else {
            self.world[parent as usize]
        };

        self.transform(transform, world);
        self.dirty[transform as usize] = true
    }

    /// Applies a given transformation matrix to the local transform.
    pub fn transform(&mut self, transform: Transform, trans_mat: Matrix4<f32>) {
        let updated = self.local[transform as usize] * trans_mat;
        self.world[transform as usize] = updated;

        let mut child = self.child[transform as usize];
        while child < Transform::max_value() {
            self.transform(child, updated);
            child = self.next_sibling[child as usize];
        }
    }

    /// Resets the dirtiness of all transforms.
    pub fn reset(&mut self) {
        for a in 0..self.entity.len() {
            self.dirty[a as usize] = false;
        }
    }

    /// Builds vectors of entities that have been modified since last reset and their transforms.
    pub fn dirty(&self, entities: &mut Vec<Entity>, transforms: &mut Vec<Matrix4<f32>>) {
        for a in 0..self.entity.len() {
            if self.dirty[a as usize] {
                entities.push(self.entity[a as usize]);
                transforms.push(self.world[a as usize]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn creating_transform() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        assert_eq!(component, 0);
    }

    #[test]
    fn destroying_transform() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        manager.destroy_transform(component);

        assert!(!manager.has_transform(entity));
    }

    #[test]
    fn checking_transform() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        assert!(!manager.has_transform(entity));
    }

    #[test]
    fn local_position() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        manager.set_local_position(entity, Vector3::new(3.0f32, 5.0f32, 0.5f32));

        assert_eq!(manager.local_position(entity), Vector3::new(3.0f32, 5.0f32, 0.5f32));
    }

    #[test]
    fn local_rotation() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        let rotation = Quaternion::from_euler_angles(PI / 4f32, PI / 4f32, PI / 4f32);
        manager.set_local_rotation(entity, rotation);

        let received = manager.local_rotation(entity);
        assert_eq!(rotation, received);
    }

    #[test]
    fn local_scale() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        let scale = Vector3::new(0.5f32, 0.5f32, 0.5f32);
        manager.set_local_scale(entity, scale);

        let received = manager.local_scale(entity);
        assert_eq!(scale, received);
    }

    #[test]
    fn world_position() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        manager.set_world_position(entity, Vector3::new(3.0f32, 5.0f32, 0.5f32));

        assert_eq!(manager.world_position(entity), Vector3::new(3.0f32, 5.0f32, 0.5f32));
    }

    #[test]
    fn world_rotation() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        let rotation = Quaternion::from_euler_angles(PI / 4f32, PI / 4f32, PI / 4f32);
        manager.set_world_rotation(entity, rotation);

        let received = manager.world_rotation(entity);
        assert_eq!(rotation, received);
    }

    #[test]
    fn linking_transforms() {
        let mut manager = SceneManager::new();
        let parent = 3 as Entity;
        let child = 5 as Entity;

        let _ = manager.create_transform(parent);
        let _ = manager.create_transform(child);

        manager.link(child, parent);
        manager.set_local_position(parent, Vector3::new(3.0f32, 5.0f32, 0.5f32));

        assert_eq!(manager.local_position(child), Vector3::from_element(0.0f32));
        assert_eq!(manager.world_position(child), Vector3::new(3.0f32, 5.0f32, 0.5f32));
    }

    #[test]
    fn unlinking_transforms() {
        let mut manager = SceneManager::new();
        let parent = 3 as Entity;
        let child = 5 as Entity;

        manager.create_transform(parent);
        manager.create_transform(child);

        manager.link(child, parent);
        manager.set_local_position(parent, Vector3::new(3.0f32, 5.0f32, 0.5f32));
        manager.unlink(child);
        manager.set_local_position(parent, Vector3::from_element(0.0f32));

        assert_eq!(manager.local_position(parent), Vector3::from_element(0.0f32));
        assert_eq!(manager.world_position(child), Vector3::new(3.0f32, 5.0f32, 0.5f32));
    }

    #[test]
    fn dirtying_transforms() {
        let mut manager = SceneManager::new();
        let entity = 3 as Entity;

        let component = manager.create_transform(entity);
        let rotation = Quaternion::from_euler_angles(PI / 4f32, PI / 4f32, PI / 4f32);
        manager.set_local_rotation(entity, rotation);

        let mut entities = Vec::new();
        let mut transforms = Vec::new();

        manager.dirty(&mut entities, &mut transforms);

        assert_eq!(entities.len(), 1);
        assert_eq!(transforms.len(), 1);
        assert_eq!(entities[0], entity);
    }
}