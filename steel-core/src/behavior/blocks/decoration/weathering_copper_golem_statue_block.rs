use crate::behavior::blocks::CopperGolemStatueBlock;
use crate::behavior::{
    BlockBehavior, BlockCollisionContext, BlockEntityCreation, BlockPlaceContext,
    InteractionResult, InventoryAccess,
};
use crate::entity::ai::path::PathComputationType;
use crate::player::Player;
use crate::world::{LevelReader, World};
use std::sync::{Arc, Weak};
use steel_macros::block_behavior;
use steel_registry::blocks::BlockRef;
use steel_registry::blocks::block_state_ext::BlockStateExt;
use steel_registry::blocks::properties::{BlockStateProperties, BoolProperty, EnumProperty};
use steel_registry::blocks::shapes::VoxelShape;
use steel_registry::item_stack::ItemStack;
use steel_registry::items::item::BlockHitResult;
use steel_registry::vanilla_items;
use steel_utils::types::InteractionHand;
use steel_utils::{BlockPos, BlockStateId, Direction};

/// Behavior for weathering copper golem statues
#[block_behavior]
pub struct WeatheringCopperGolemStatueBlock {
    block: BlockRef,
    copper_golem_statue_block: CopperGolemStatueBlock,
}

const WATERLOGGED: &BoolProperty = &BlockStateProperties::WATERLOGGED;
const FACING: &EnumProperty<Direction> = &BlockStateProperties::HORIZONTAL_FACING;

impl WeatheringCopperGolemStatueBlock {
    /// creates a weathering (non waxed) copper golem statue
    #[must_use]
    pub const fn new(block: BlockRef) -> Self {
        Self {
            block,
            copper_golem_statue_block: CopperGolemStatueBlock::new(block),
        }
    }
}

impl BlockBehavior for WeatheringCopperGolemStatueBlock {
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
        moved_by_piston: bool,
    ) {
        self.copper_golem_statue_block
            .affect_neighbors_after_removal(state, world, pos, moved_by_piston)
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
        //todo was feature
        self.copper_golem_statue_block
            .use_item_on(state, world, pos, player, hand, hit_result, inv)
    }

    fn get_clone_item_stack(
        &self,
        _block: BlockRef,
        _state: BlockStateId,
        _include_data: bool,
    ) -> Option<ItemStack> {
        Some(
            //todo copy pose & weathering state
            ItemStack::new(&vanilla_items::COPPER_GOLEM_STATUE),
        )
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

    fn new_block_entity(
        &self,
        level: Weak<World>,
        pos: BlockPos,
        state: BlockStateId,
    ) -> BlockEntityCreation {
        self.copper_golem_statue_block
            .new_block_entity(level, pos, state)
    }

    fn should_keep_block_entity(&self, old_state: BlockStateId, new_state: BlockStateId) -> bool {
        self.copper_golem_statue_block
            .should_keep_block_entity(old_state, new_state)
    }

    fn has_analog_output_signal(&self, state: BlockStateId) -> bool {
        self.copper_golem_statue_block.has_analog_output_signal(state)
    }

    fn get_analog_output_signal(&self, state: BlockStateId, world: &dyn LevelReader, pos: BlockPos, direction: Direction) -> i32 {
        self.copper_golem_statue_block.get_analog_output_signal(state, world, pos, direction)
    }

    // TODO weathering state + ticking
}
