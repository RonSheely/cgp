use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::traits::has_field::HasField;
use crate::FieldGetter;

#[diagnostic::on_unimplemented(
    message = "HasFieldMut is not implemented for {Self} with the field: {Tag}",
    note = "You need to add #[derive(HasField)] to {Self} with the given field present in the struct"
)]
pub trait HasFieldMut<Tag>: HasField<Tag> {
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value;
}

pub trait MutFieldGetter<Context, Tag>: FieldGetter<Context, Tag> {
    fn get_field_mut(context: &mut Context, tag: PhantomData<Tag>) -> &mut Self::Value;
}

impl<Context, Tag, Target, Value> HasFieldMut<Tag> for Context
where
    Context: DerefMut<Target = Target>,
    Target: HasFieldMut<Tag, Value = Value> + 'static,
{
    fn get_field_mut(&mut self, tag: PhantomData<Tag>) -> &mut Self::Value {
        self.deref_mut().get_field_mut(tag)
    }
}
