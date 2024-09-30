#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anims::{CircleAnim, LennyAnim};
use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, window::WindowResolution,
};
use bevy_2delight_anims::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod anims {
    use bevy::{reflect::Reflect, render::view::RenderLayers};
    use bevy_2delight_anims::prelude::*;
    use bevy_2delight_anims_macros::AnimStateMachine;
    struct MainLayer;
    impl Into<RenderLayers> for MainLayer {
        fn into(self) -> RenderLayers {
            RenderLayers::from_layers(&[1])
        }
    }
    struct OtherLayer;
    impl Into<RenderLayers> for OtherLayer {
        fn into(self) -> RenderLayers {
            RenderLayers::from_layers(&[2])
        }
    }
    pub enum CircleAnim {
        #[default]
        #[file("platformer/circle.png")]
        #[size(24, 24)]
        #[length(8)]
        #[fps(20.0)]
        #[zix(10.0)]
        #[render_layers(MainLayer, OtherLayer)]
        Spin,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CircleAnim {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "Spin")
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CircleAnim {}
    #[automatically_derived]
    impl ::core::clone::Clone for CircleAnim {
        #[inline]
        fn clone(&self) -> CircleAnim {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for CircleAnim {
        #[inline]
        fn default() -> CircleAnim {
            Self::Spin
        }
    }
    const _: () = {
        #[allow(unused_mut)]
        impl bevy::reflect::GetTypeRegistration for CircleAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn get_type_registration() -> bevy::reflect::TypeRegistration {
                let mut registration = bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        bevy::reflect::ReflectFromPtr,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        bevy::reflect::ReflectFromReflect,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {}
        }
        impl bevy::reflect::Typed for CircleAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_info() -> &'static bevy::reflect::TypeInfo {
                static CELL: bevy::reflect::utility::NonGenericTypeInfoCell = bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    bevy::reflect::TypeInfo::Enum(
                        bevy::reflect::EnumInfo::new::<
                            Self,
                        >(
                                &[
                                    bevy::reflect::VariantInfo::Unit(
                                        bevy::reflect::UnitVariantInfo::new("Spin")
                                            .with_custom_attributes(
                                                bevy::reflect::attributes::CustomAttributes::default(),
                                            ),
                                    ),
                                ],
                            )
                            .with_custom_attributes(
                                bevy::reflect::attributes::CustomAttributes::default(),
                            ),
                    )
                })
            }
        }
        impl bevy::reflect::TypePath for CircleAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_path() -> &'static str {
                "platformer::anims::CircleAnim"
            }
            fn short_type_path() -> &'static str {
                "CircleAnim"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("CircleAnim")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "platformer::anims".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("platformer::anims")
            }
        }
        impl bevy::reflect::Enum for CircleAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn field(
                &self,
                __name_param: &str,
            ) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                __index_param: usize,
            ) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                __name_param: &str,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                __index_param: usize,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn index_of(&self, __name_param: &str) -> ::core::option::Option<usize> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, __index_param: usize) -> ::core::option::Option<&str> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                bevy::reflect::VariantFieldIter::new(self)
            }
            #[inline]
            fn field_len(&self) -> usize {
                match self {
                    CircleAnim::Spin { .. } => 0usize,
                    _ => 0,
                }
            }
            #[inline]
            fn variant_name(&self) -> &str {
                match self {
                    CircleAnim::Spin { .. } => "Spin",
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_index(&self) -> usize {
                match self {
                    CircleAnim::Spin { .. } => 0usize,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_type(&self) -> bevy::reflect::VariantType {
                match self {
                    CircleAnim::Spin { .. } => bevy::reflect::VariantType::Unit,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                bevy::reflect::DynamicEnum::from_ref::<Self>(self)
            }
        }
        impl bevy::reflect::Reflect for CircleAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as bevy::reflect::Typed>::type_info())
            }
            #[inline]
            fn into_any(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn clone_value(&self) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
            }
            #[inline]
            fn set(
                &mut self,
                __value_param: ::std::boxed::Box<dyn bevy::reflect::Reflect>,
            ) -> ::core::result::Result<
                (),
                ::std::boxed::Box<dyn bevy::reflect::Reflect>,
            > {
                *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn try_apply(
                &mut self,
                __value_param: &dyn bevy::reflect::Reflect,
            ) -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                if let bevy::reflect::ReflectRef::Enum(__value_param) = bevy::reflect::Reflect::reflect_ref(
                    __value_param,
                ) {
                    if bevy::reflect::Enum::variant_name(self)
                        == bevy::reflect::Enum::variant_name(__value_param)
                    {
                        match bevy::reflect::Enum::variant_type(__value_param) {
                            bevy::reflect::VariantType::Struct => {
                                for field in bevy::reflect::Enum::iter_fields(
                                    __value_param,
                                ) {
                                    let name = field.name().unwrap();
                                    if let ::core::option::Option::Some(v) = bevy::reflect::Enum::field_mut(
                                        self,
                                        name,
                                    ) {
                                        bevy::reflect::Reflect::try_apply(v, field.value())?;
                                    }
                                }
                            }
                            bevy::reflect::VariantType::Tuple => {
                                for (index, field) in ::core::iter::Iterator::enumerate(
                                    bevy::reflect::Enum::iter_fields(__value_param),
                                ) {
                                    if let ::core::option::Option::Some(v) = bevy::reflect::Enum::field_at_mut(
                                        self,
                                        index,
                                    ) {
                                        bevy::reflect::Reflect::try_apply(v, field.value())?;
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match bevy::reflect::Enum::variant_name(__value_param) {
                            "Spin" => *self = CircleAnim::Spin {},
                            name => {
                                return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                    enum_name: ::core::convert::Into::into(
                                        bevy::reflect::DynamicTypePath::reflect_type_path(self),
                                    ),
                                    variant_name: ::core::convert::Into::into(name),
                                });
                            }
                        }
                    }
                } else {
                    return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                        to_kind: bevy::reflect::ReflectKind::Enum,
                    });
                }
                ::core::result::Result::Ok(())
            }
            fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                bevy::reflect::ReflectKind::Enum
            }
            fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                bevy::reflect::ReflectRef::Enum(self)
            }
            fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                bevy::reflect::ReflectMut::Enum(self)
            }
            fn reflect_owned(
                self: ::std::boxed::Box<Self>,
            ) -> bevy::reflect::ReflectOwned {
                bevy::reflect::ReflectOwned::Enum(self)
            }
            fn reflect_hash(&self) -> ::core::option::Option<u64> {
                bevy::reflect::enum_hash(self)
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn bevy::reflect::Reflect,
            ) -> ::core::option::Option<bool> {
                bevy::reflect::enum_partial_eq(self, value)
            }
        }
        impl bevy::reflect::FromReflect for CircleAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn from_reflect(
                __param0: &dyn bevy::reflect::Reflect,
            ) -> ::core::option::Option<Self> {
                if let bevy::reflect::ReflectRef::Enum(__param0) = bevy::reflect::Reflect::reflect_ref(
                    __param0,
                ) {
                    match bevy::reflect::Enum::variant_name(__param0) {
                        "Spin" => ::core::option::Option::Some(CircleAnim::Spin {}),
                        name => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "variant with name `{0}` does not exist on enum `{1}`",
                                    name,
                                    <Self as bevy::reflect::TypePath>::type_path(),
                                ),
                            );
                        }
                    }
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CircleAnim {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CircleAnim {
        #[inline]
        fn eq(&self, other: &CircleAnim) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for CircleAnim {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for CircleAnim {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    impl bevy_2delight_anims::prelude::AnimStateMachine for CircleAnim {
        fn all() -> Vec<Self> {
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([Self::Spin]))
        }
        fn get_body(&self) -> AnimBody {
            match self {
                Self::Spin => {
                    bevy_2delight_anims::prelude::AnimBody::new(
                            "platformer/circle.png",
                            24u32,
                            24u32,
                        )
                        .with_length(8u32)
                        .with_fps(Some(20f32 as f32))
                        .with_offset(0f32, 0f32)
                        .with_zix(10f32)
                        .add_render_layers(MainLayer.into())
                        .add_render_layers(OtherLayer.into())
                }
            }
        }
        fn get_next(&self) -> AnimNextState<Self> {
            match self {
                Self::Spin => bevy_2delight_anims::prelude::AnimNextState::Stay,
            }
        }
    }
    pub enum LennyAnim {
        #[file("platformer/lenny_idle.png")]
        #[size(17, 17)]
        #[offset(-0.5, -0.5)]
        Idle,
        #[default]
        #[file("platformer/lenny_run.png")]
        #[size(17, 17)]
        #[length(5)]
        #[offset(-10.5, -0.5)]
        #[next(RunOffset)]
        Run,
        #[file("platformer/lenny_run.png")]
        #[size(17, 17)]
        #[length(5)]
        #[offset(10.5, -0.5)]
        #[next(Run)]
        RunOffset,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LennyAnim {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    LennyAnim::Idle => "Idle",
                    LennyAnim::Run => "Run",
                    LennyAnim::RunOffset => "RunOffset",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for LennyAnim {}
    #[automatically_derived]
    impl ::core::clone::Clone for LennyAnim {
        #[inline]
        fn clone(&self) -> LennyAnim {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for LennyAnim {
        #[inline]
        fn default() -> LennyAnim {
            Self::Run
        }
    }
    const _: () = {
        #[allow(unused_mut)]
        impl bevy::reflect::GetTypeRegistration for LennyAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn get_type_registration() -> bevy::reflect::TypeRegistration {
                let mut registration = bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        bevy::reflect::ReflectFromPtr,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        bevy::reflect::ReflectFromReflect,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {}
        }
        impl bevy::reflect::Typed for LennyAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_info() -> &'static bevy::reflect::TypeInfo {
                static CELL: bevy::reflect::utility::NonGenericTypeInfoCell = bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    bevy::reflect::TypeInfo::Enum(
                        bevy::reflect::EnumInfo::new::<
                            Self,
                        >(
                                &[
                                    bevy::reflect::VariantInfo::Unit(
                                        bevy::reflect::UnitVariantInfo::new("Idle")
                                            .with_custom_attributes(
                                                bevy::reflect::attributes::CustomAttributes::default(),
                                            ),
                                    ),
                                    bevy::reflect::VariantInfo::Unit(
                                        bevy::reflect::UnitVariantInfo::new("Run")
                                            .with_custom_attributes(
                                                bevy::reflect::attributes::CustomAttributes::default(),
                                            ),
                                    ),
                                    bevy::reflect::VariantInfo::Unit(
                                        bevy::reflect::UnitVariantInfo::new("RunOffset")
                                            .with_custom_attributes(
                                                bevy::reflect::attributes::CustomAttributes::default(),
                                            ),
                                    ),
                                ],
                            )
                            .with_custom_attributes(
                                bevy::reflect::attributes::CustomAttributes::default(),
                            ),
                    )
                })
            }
        }
        impl bevy::reflect::TypePath for LennyAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn type_path() -> &'static str {
                "platformer::anims::LennyAnim"
            }
            fn short_type_path() -> &'static str {
                "LennyAnim"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("LennyAnim")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "platformer::anims".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("platformer::anims")
            }
        }
        impl bevy::reflect::Enum for LennyAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn field(
                &self,
                __name_param: &str,
            ) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                __index_param: usize,
            ) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                __name_param: &str,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                __index_param: usize,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn index_of(&self, __name_param: &str) -> ::core::option::Option<usize> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, __index_param: usize) -> ::core::option::Option<&str> {
                match self {
                    _ => ::core::option::Option::None,
                }
            }
            fn iter_fields(&self) -> bevy::reflect::VariantFieldIter {
                bevy::reflect::VariantFieldIter::new(self)
            }
            #[inline]
            fn field_len(&self) -> usize {
                match self {
                    LennyAnim::Idle { .. } => 0usize,
                    LennyAnim::Run { .. } => 0usize,
                    LennyAnim::RunOffset { .. } => 0usize,
                    _ => 0,
                }
            }
            #[inline]
            fn variant_name(&self) -> &str {
                match self {
                    LennyAnim::Idle { .. } => "Idle",
                    LennyAnim::Run { .. } => "Run",
                    LennyAnim::RunOffset { .. } => "RunOffset",
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_index(&self) -> usize {
                match self {
                    LennyAnim::Idle { .. } => 0usize,
                    LennyAnim::Run { .. } => 1usize,
                    LennyAnim::RunOffset { .. } => 2usize,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            #[inline]
            fn variant_type(&self) -> bevy::reflect::VariantType {
                match self {
                    LennyAnim::Idle { .. } => bevy::reflect::VariantType::Unit,
                    LennyAnim::Run { .. } => bevy::reflect::VariantType::Unit,
                    LennyAnim::RunOffset { .. } => bevy::reflect::VariantType::Unit,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
            fn clone_dynamic(&self) -> bevy::reflect::DynamicEnum {
                bevy::reflect::DynamicEnum::from_ref::<Self>(self)
            }
        }
        impl bevy::reflect::Reflect for LennyAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as bevy::reflect::Typed>::type_info())
            }
            #[inline]
            fn into_any(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::std::boxed::Box<Self>,
            ) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn clone_value(&self) -> ::std::boxed::Box<dyn bevy::reflect::Reflect> {
                ::std::boxed::Box::new(bevy::reflect::Enum::clone_dynamic(self))
            }
            #[inline]
            fn set(
                &mut self,
                __value_param: ::std::boxed::Box<dyn bevy::reflect::Reflect>,
            ) -> ::core::result::Result<
                (),
                ::std::boxed::Box<dyn bevy::reflect::Reflect>,
            > {
                *self = <dyn bevy::reflect::Reflect>::take(__value_param)?;
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn try_apply(
                &mut self,
                __value_param: &dyn bevy::reflect::Reflect,
            ) -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                if let bevy::reflect::ReflectRef::Enum(__value_param) = bevy::reflect::Reflect::reflect_ref(
                    __value_param,
                ) {
                    if bevy::reflect::Enum::variant_name(self)
                        == bevy::reflect::Enum::variant_name(__value_param)
                    {
                        match bevy::reflect::Enum::variant_type(__value_param) {
                            bevy::reflect::VariantType::Struct => {
                                for field in bevy::reflect::Enum::iter_fields(
                                    __value_param,
                                ) {
                                    let name = field.name().unwrap();
                                    if let ::core::option::Option::Some(v) = bevy::reflect::Enum::field_mut(
                                        self,
                                        name,
                                    ) {
                                        bevy::reflect::Reflect::try_apply(v, field.value())?;
                                    }
                                }
                            }
                            bevy::reflect::VariantType::Tuple => {
                                for (index, field) in ::core::iter::Iterator::enumerate(
                                    bevy::reflect::Enum::iter_fields(__value_param),
                                ) {
                                    if let ::core::option::Option::Some(v) = bevy::reflect::Enum::field_at_mut(
                                        self,
                                        index,
                                    ) {
                                        bevy::reflect::Reflect::try_apply(v, field.value())?;
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        match bevy::reflect::Enum::variant_name(__value_param) {
                            "Idle" => *self = LennyAnim::Idle {},
                            "Run" => *self = LennyAnim::Run {},
                            "RunOffset" => *self = LennyAnim::RunOffset {},
                            name => {
                                return ::core::result::Result::Err(bevy::reflect::ApplyError::UnknownVariant {
                                    enum_name: ::core::convert::Into::into(
                                        bevy::reflect::DynamicTypePath::reflect_type_path(self),
                                    ),
                                    variant_name: ::core::convert::Into::into(name),
                                });
                            }
                        }
                    }
                } else {
                    return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: bevy::reflect::Reflect::reflect_kind(__value_param),
                        to_kind: bevy::reflect::ReflectKind::Enum,
                    });
                }
                ::core::result::Result::Ok(())
            }
            fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                bevy::reflect::ReflectKind::Enum
            }
            fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                bevy::reflect::ReflectRef::Enum(self)
            }
            fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                bevy::reflect::ReflectMut::Enum(self)
            }
            fn reflect_owned(
                self: ::std::boxed::Box<Self>,
            ) -> bevy::reflect::ReflectOwned {
                bevy::reflect::ReflectOwned::Enum(self)
            }
            fn reflect_hash(&self) -> ::core::option::Option<u64> {
                bevy::reflect::enum_hash(self)
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn bevy::reflect::Reflect,
            ) -> ::core::option::Option<bool> {
                bevy::reflect::enum_partial_eq(self, value)
            }
        }
        impl bevy::reflect::FromReflect for LennyAnim
        where
            Self: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn from_reflect(
                __param0: &dyn bevy::reflect::Reflect,
            ) -> ::core::option::Option<Self> {
                if let bevy::reflect::ReflectRef::Enum(__param0) = bevy::reflect::Reflect::reflect_ref(
                    __param0,
                ) {
                    match bevy::reflect::Enum::variant_name(__param0) {
                        "Idle" => ::core::option::Option::Some(LennyAnim::Idle {}),
                        "Run" => ::core::option::Option::Some(LennyAnim::Run {}),
                        "RunOffset" => {
                            ::core::option::Option::Some(LennyAnim::RunOffset {})
                        }
                        name => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "variant with name `{0}` does not exist on enum `{1}`",
                                    name,
                                    <Self as bevy::reflect::TypePath>::type_path(),
                                ),
                            );
                        }
                    }
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for LennyAnim {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LennyAnim {
        #[inline]
        fn eq(&self, other: &LennyAnim) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for LennyAnim {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for LennyAnim {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl bevy_2delight_anims::prelude::AnimStateMachine for LennyAnim {
        fn all() -> Vec<Self> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([Self::Idle, Self::Run, Self::RunOffset]),
            )
        }
        fn get_body(&self) -> AnimBody {
            match self {
                Self::Idle => {
                    bevy_2delight_anims::prelude::AnimBody::new(
                            "platformer/lenny_idle.png",
                            17u32,
                            17u32,
                        )
                        .with_length(1u32)
                        .with_fps(None)
                        .with_offset(-0.5f32, -0.5f32)
                        .with_zix(0f32)
                }
                Self::Run => {
                    bevy_2delight_anims::prelude::AnimBody::new(
                            "platformer/lenny_run.png",
                            17u32,
                            17u32,
                        )
                        .with_length(5u32)
                        .with_fps(None)
                        .with_offset(-10.5f32, -0.5f32)
                        .with_zix(0f32)
                }
                Self::RunOffset => {
                    bevy_2delight_anims::prelude::AnimBody::new(
                            "platformer/lenny_run.png",
                            17u32,
                            17u32,
                        )
                        .with_length(5u32)
                        .with_fps(None)
                        .with_offset(10.5f32, -0.5f32)
                        .with_zix(0f32)
                }
            }
        }
        fn get_next(&self) -> AnimNextState<Self> {
            match self {
                Self::Idle => bevy_2delight_anims::prelude::AnimNextState::Stay,
                Self::Run => {
                    bevy_2delight_anims::prelude::AnimNextState::Some(Self::RunOffset)
                }
                Self::RunOffset => {
                    bevy_2delight_anims::prelude::AnimNextState::Some(Self::Run)
                }
            }
        }
    }
}
fn main() {
    let mut app = App::new();
    use bevy::asset::AssetMetaCheck;
    app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        title: "PLATFORMER".to_string(),
                        resolution: WindowResolution::new(320.0 * 3.0, 180.0 * 3.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(
            WorldInspectorPlugin::default()
                .run_if(input_toggle_active(false, KeyCode::Tab)),
        );
    app.add_plugins(AnimPlugin {
        default_fps: 4.0,
        ..default()
    });
    app.add_plugins(AnimDefnPlugin::<CircleAnim>::default());
    app.add_plugins(AnimDefnPlugin::<LennyAnim>::default());
    app.add_systems(Startup, startup);
    app.add_systems(Update, flips);
    app.run();
}
fn startup(mut commands: Commands) {
    commands.spawn((Name::new("camera"), Camera2dBundle::default()));
    commands.spawn((Name::new("sanity_sprite"), SpriteBundle::default()));
    commands
        .spawn((
            Name::new("lenny"),
            AnimMan::<CircleAnim>::default(),
            AnimMan::<LennyAnim>::default(),
            SpatialBundle::from_transform(Transform::from_scale(Vec3::ONE * 4.0)),
        ));
}
fn flips() {}
fn lenny_anim_state_change() {}
