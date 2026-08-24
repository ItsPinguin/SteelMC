use crate::behavior::blocks::{WeatherState, WeatheringCopper};
use crate::behavior::{
    BlockBehavior, BlockCollisionContext, BlockEntityCreation, BlockPlaceContext,
    InteractionResult, InventoryAccess,
};
use crate::block_entity::entities::CopperGolemStatueBlockEntity;
use crate::entity::ai::path::PathComputationType;
use crate::player::Player;
use crate::world::{LevelReader, World};
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use steel_macros::block_behavior;
use steel_registry::REGISTRY;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{
    BlockStateProperties, BoolProperty, EnumProperty, Pose, PropertyEnum,
};
use steel_registry::blocks::shapes::VoxelShape;
use steel_registry::data_components::components::BlockItemStateProperties;
use steel_registry::data_components::vanilla_components::BLOCK_STATE;
use steel_registry::fluid::FluidStateExt;
use steel_registry::item_stack::ItemStack;
use steel_registry::items::item::BlockHitResult;
use steel_registry::vanilla_block_tags::BlockTag;
use steel_registry::vanilla_item_tags::ItemTag;
use steel_registry::vanilla_items::HONEYCOMB;
use steel_utils::types::{InteractionHand, UpdateFlags};
use steel_utils::{BlockLocalAabb, BlockPos, BlockStateId, Direction};

/// Behavior for copper golem statues
#[block_behavior]
pub struct CopperGolemStatueBlock {
    block: BlockRef,
}

const FACING: &EnumProperty<Direction> = &BlockStateProperties::HORIZONTAL_FACING;
const POSE: &EnumProperty<Pose> = &BlockStateProperties::COPPER_GOLEM_POSE;
const WATERLOGGED: &BoolProperty = &BlockStateProperties::WATERLOGGED;
const SHAPE_BOXES: &[BlockLocalAabb] = &[BlockLocalAabb::new(
    0.1875, 0.0, 0.1875, 0.8125, 0.875, 0.8125,
)];
const SHAPE: VoxelShape = VoxelShape::from_boxes(SHAPE_BOXES);
//const WEATHER_STATE: WeatherState = WeatherState::Unaffected;

impl CopperGolemStatueBlock {
    /// creates a waxed copper golem statue
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self { block }
    }

    fn update_pose(world: &Arc<World>, state: BlockStateId, pos: BlockPos, _player: &Player) {
        world.set_block(
            pos,
            state.set_value(POSE, state.get_value(POSE).get_next_pose()),
            UpdateFlags::UPDATE_ALL,
        );
    }
}

impl BlockBehavior for CopperGolemStatueBlock {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        context.with_item(|item| {
            let pose_str = item
                .get(BLOCK_STATE)
                .unwrap()
                .get("copper_golem_pose").unwrap();
            let pose: Pose = match pose_str {
                "sitting" => Pose::Sitting,
                "running" => Pose::Running,
                "star" => Pose::Star,
                _ => Pose::Standing,
            };
            Some(
                self.block
                    .default_state()
                    .set_value(WATERLOGGED, context.is_water_source())
                    .set_value(FACING, context.horizontal_direction().opposite())
                    .set_value(POSE, pose),
            )
        })
    }

    fn affect_neighbors_after_removal(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        _moved_by_piston: bool,
    ) {
        world.update_neighbor_for_output_signal(pos, state.get_block());
    }

    fn use_item_on(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        player: &Player,
        _hand: InteractionHand,
        _hit_result: &BlockHitResult,
        inv: &mut InventoryAccess,
    ) -> InteractionResult {
        if inv.with_item(|item_stack| item_stack.item.has_tag(&ItemTag::AXES)) {
            return InteractionResult::Pass;
        }

        Self::update_pose(world, state, pos, player);
        InteractionResult::Success
    }

    fn get_clone_item_stack(
        &self,
        block: BlockRef,
        state: BlockStateId,
        _include_data: bool,
    ) -> Option<ItemStack> {
        let pose: Pose = state.get_value(POSE);
        let block_state = BlockItemStateProperties::new(BTreeMap::from([(
            "copper_golem_pose".to_owned(),
            pose.as_str().into(),
        )]));

        let stack = REGISTRY.items.by_block(block);
        let mut item = ItemStack::new(stack);
        item.set(BLOCK_STATE, block_state);
        Some(item)
    }

    fn is_pathfindable(&self, state: BlockStateId, computation_type: PathComputationType) -> bool {
        computation_type == PathComputationType::Water && state.get_fluid_state().is_water()
    }

    fn get_collision_shape(
        &self,
        _state: BlockStateId,
        _world: &dyn LevelReader,
        _pos: BlockPos,
        _context: BlockCollisionContext,
    ) -> VoxelShape {
        SHAPE
    }

    fn new_block_entity(
        &self,
        level: Weak<World>,
        pos: BlockPos,
        state: BlockStateId,
    ) -> BlockEntityCreation {
        BlockEntityCreation::Created(Arc::new(CopperGolemStatueBlockEntity::new(
            level, pos, state,
        )))
    }

    fn should_keep_block_entity(&self, old_state: BlockStateId, _new_state: BlockStateId) -> bool {
        old_state
            .get_block()
            .has_tag(&BlockTag::COPPER_GOLEM_STATUES)
    }

    fn has_analog_output_signal(&self, _state: BlockStateId) -> bool {
        true
    }

    fn get_analog_output_signal(
        &self,
        state: BlockStateId,
        _world: &dyn LevelReader,
        _pos: BlockPos,
        _direction: Direction,
    ) -> i32 {
        match state.get_value(POSE) {
            Pose::Standing => 1,
            Pose::Sitting => 2,
            Pose::Running => 3,
            Pose::Star => 4,
        }
    }
}

/// Behavior for weathering copper golem statues
#[block_behavior]
pub struct WeatheringCopperGolemStatueBlock {
    copper_golem_statue_block: CopperGolemStatueBlock,
    #[json_arg(r#enum = "WeatherState", json = "weathering_state")]
    weathering: WeatheringCopper,
}

impl WeatheringCopperGolemStatueBlock {
    /// creates a weathering (non waxed) copper golem statue
    #[must_use]
    pub const fn new(block: BlockRef, weather_state: WeatherState) -> Self {
        Self {
            copper_golem_statue_block: CopperGolemStatueBlock::new(block),
            weathering: WeatheringCopper::new(weather_state),
        }
    }
}

impl BlockBehavior for WeatheringCopperGolemStatueBlock {
    fn get_state_for_placement(&self, context: &BlockPlaceContext<'_>) -> Option<BlockStateId> {
        self.copper_golem_statue_block
            .get_state_for_placement(context)
    }

    fn affect_neighbors_after_removal(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        moved_by_piston: bool,
    ) {
        self.copper_golem_statue_block
            .affect_neighbors_after_removal(state, world, pos, moved_by_piston);
    }

    fn use_item_on(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        player: &Player,
        hand: InteractionHand,
        hit_result: &BlockHitResult,
        inv: &mut InventoryAccess,
    ) -> InteractionResult {
        if inv.with_item(|item_stack| !item_stack.item.has_tag(&ItemTag::AXES)) {
            if inv.with_item(|item_stack| { item_stack.item.key == HONEYCOMB.key }) {
                return InteractionResult::Pass;
            }
            return self.copper_golem_statue_block
                    .use_item_on(state, world, pos, player, hand, hit_result, inv)
        }

        if self.weathering.get_weather_state() == WeatherState::Unaffected {
            // TODO: spawn entity
        }
        InteractionResult::Pass
    }

    fn get_clone_item_stack(
        &self,
        block: BlockRef,
        state: BlockStateId,
        include_data: bool,
    ) -> Option<ItemStack> {
        self.copper_golem_statue_block
            .get_clone_item_stack(block, state, include_data)
    }

    fn is_pathfindable(&self, state: BlockStateId, computation_type: PathComputationType) -> bool {
        self.copper_golem_statue_block
            .is_pathfindable(state, computation_type)
    }

    fn get_collision_shape(
        &self,
        state: BlockStateId,
        world: &dyn LevelReader,
        pos: BlockPos,
        context: BlockCollisionContext,
    ) -> VoxelShape {
        self.copper_golem_statue_block
            .get_collision_shape(state, world, pos, context)
    }

    fn random_tick(&self, state: BlockStateId, world: &Arc<World>, pos: BlockPos) {
        self.weathering.change_over_time(state, world, pos);
    }

    fn has_analog_output_signal(&self, state: BlockStateId) -> bool {
        self.copper_golem_statue_block
            .has_analog_output_signal(state)
    }

    // TODO weathering state + ticking

    fn get_analog_output_signal(
        &self,
        state: BlockStateId,
        world: &dyn LevelReader,
        pos: BlockPos,
        direction: Direction,
    ) -> i32 {
        self.copper_golem_statue_block
            .get_analog_output_signal(state, world, pos, direction)
    }
}
