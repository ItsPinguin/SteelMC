use std::sync::Weak;
use crate::block_entity::{BlockEntity, BlockEntityBase};
use simdnbt::borrow::BaseNbtCompound as BorrowedNbtCompound;
use simdnbt::owned::NbtCompound;
use steel_registry::vanilla_block_entity_types;
use steel_utils::{BlockPos, BlockStateId, DowncastType, DowncastTypeKey};
use crate::world::World;

/// copper golem statue block entity
pub struct CopperGolemStatueBlockEntity {
    pub(crate) base: BlockEntityBase
}

impl CopperGolemStatueBlockEntity {
    /// creates copper golem statue block entity
    #[must_use]
    pub fn new(level: Weak<World>, pos: BlockPos, state: BlockStateId) -> Self {
        Self {
            base: BlockEntityBase::new(&vanilla_block_entity_types::COPPER_GOLEM_STATUE, level, pos, state),
        }
    }
}

unsafe impl DowncastType for CopperGolemStatueBlockEntity {
    const TYPE_KEY: DowncastTypeKey = DowncastTypeKey::new("steel:block_entity/copper_golem");
}

impl BlockEntity for CopperGolemStatueBlockEntity {
    fn base(&self) -> &BlockEntityBase {
        &self.base
    }

    fn load_additional(&self, _nbt: &BorrowedNbtCompound<'_>) {}

    fn save_additional(&self, _nbt: &mut NbtCompound) {}

    fn get_update_tag(&self) -> Option<NbtCompound> {
        Some(NbtCompound::new())
    }
}