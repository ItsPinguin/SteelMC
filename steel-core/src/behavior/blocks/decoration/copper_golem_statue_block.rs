use crate::behavior::{
    BlockBehavior, BlockCollisionContext, BlockEntityCreation, BlockPlaceContext,
    InteractionResult, InventoryAccess,
};
use crate::block_entity::entities::CopperGolemStatueBlockEntity;
use crate::entity::ai::path::PathComputationType;
use crate::player::Player;
use crate::world::{LevelReader, World};
use std::sync::{Arc, Weak};
use steel_macros::block_behavior;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{BlockStateProperties, BoolProperty, EnumProperty, Pose};
use steel_registry::blocks::shapes::VoxelShape;
use steel_registry::fluid::FluidStateExt;
use steel_registry::item_stack::ItemStack;
use steel_registry::items::item::BlockHitResult;
use steel_registry::vanilla_block_tags::BlockTag;
use steel_registry::vanilla_item_tags::ItemTag;
use steel_registry::vanilla_items;
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
        Some(
            self.block
                .default_state()
                .set_value(WATERLOGGED, context.is_water_source())
                .set_value(FACING, context.horizontal_direction().opposite()),
        )
    }

    fn affect_neighbors_after_removal(
        &self,
        state: BlockStateId,
        world: &Arc<World>,
        pos: BlockPos,
        _moved_by_piston: bool,
    ) {
        world.update_neighbor_for_output_signal(pos, state.get_block())
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
        _block: BlockRef,
        _state: BlockStateId,
        _include_data: bool,
    ) -> Option<ItemStack> {
        Some(
            //todo copy pose & weathering state
            ItemStack::new(&vanilla_items::WAXED_COPPER_GOLEM_STATUE),
        )
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

    //fn update_shape(
    //    &self,
    //    state: BlockStateId,
    //    world: &dyn ScheduledTickAccess,
    //    pos: BlockPos,
    //    _direction: Direction,
    //    _neighbor_pos: BlockPos,
    //    _neighbor_state: BlockStateId,
    //) -> BlockStateId {
    //    if state.get_value(WATERLOGGED) {
    //        return world.schedule_fluid_tick_default(pos, &WATER, &*WATER.tick_delay);
    //    }
    //    BlockBehavior::update_shape(state, world, pos, _direction, _neighbor_pos, _neighbor_state)
    //}
}
