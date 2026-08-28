use super::super::test;
use crate::components::ui;
use crate::configs::material;
use super::super::global;

use bevy::prelude::*;

const STDSIZE:f32 = 32.;

pub fn head() -> impl Scene {
    let rown = 6.;//
    let perh = 15.;
    let rowl =[1.,4.];
    let coll =[20.,25.,20.,35.];
    let minh = STDSIZE*2.;
    let maxh = minh*2.;
    bsn! {
        Node{
            display:Display::Flex,
            position_type: PositionType::Absolute,
            left: Val::Percent(0.),
            top: Val::Percent(0.),
            height: Val::Percent(perh),
            min_height: Val::Px(minh),
            max_height: Val::Px(maxh),
            justify_self: JustifySelf::Center,
            aspect_ratio: {Some(rown)},
        }
        Children [(
            ui::CUIhead
            Node{
                display:Display::Flex,
                height:Val::Percent(100.),
                aspect_ratio: {Some(rowl[0])},
            }
            ImageNode{
                image:format!("imgs/Headers/{}.png",5000),
                image_mode:NodeImageMode::Stretch,
            }),(
            Node{
                height:Val::Percent(100.),
                aspect_ratio: {Some(rowl[1])},
                flex_direction: FlexDirection::Column
            }
            Children [(
                Node{
                    height:Val::Percent(coll[0]),
                    width:Val::Percent(100.)
                }
                ImageNode{
                    image:format!("imgs/Headers/{}.png",5000),
                    image_mode:NodeImageMode::Stretch,
                }
            ),(
                ui::CUIhpbar
                Node{
                    height:Val::Percent(coll[1]),
                    width:Val::Percent(100.)
                }
                MaterialNode<material::HpMaterial>(
                    asset_value(material::HpMaterial { color: LinearRgba::WHITE }))
            ),(
                Node{
                    height:Val::Percent(coll[2]),
                    width:Val::Percent(100.)
                }
                ImageNode{
                    image:format!("imgs/Headers/{}.png",5000),
                    image_mode:NodeImageMode::Stretch,
                }
            ),(
                Node{
                    height:Val::Percent(coll[3]),
                    width:Val::Percent(100.)
                }
                ImageNode{
                    image:format!("imgs/Headers/{}.png",5000),
                    image_mode:NodeImageMode::Stretch,
                }
            )]
        )]
    }
}

pub fn map() -> impl Scene {
    let perh = 20.;
    let minh =STDSIZE*3.;
    let maxh = minh*4.;
    bsn! {
    ui::CUImap
    #Emap
    Node{
        display:Display::Flex,
        position_type: PositionType::Absolute,
        align_items: AlignItems::FlexStart,
        flex_direction: FlexDirection::Column,
        right: Val::Px(0.),
        top: Val::Px(0.),
        width: Val::Percent(perh),
        min_width: Val::Px(minh),
        min_height: Val::Px(minh),
        max_width: Val::Px(maxh),
        max_height: Val::Px(maxh),
        aspect_ratio: {Some(1.)}}
    MaterialNode<material::MapMaterial>(
        asset_value(material::MapMaterial { color: LinearRgba::WHITE }))}
}

pub fn info(txt: String) -> impl Scene {
    bsn! {
        ui::CUIinfo
        #Einfo
        Node{
            height: Val::Percent(60.),
            width: Val::Percent(40.),
            position_type: PositionType::Absolute,
            left: Val::Px(0.),
            top: Val::Px(0.),
        }
        Children [
        Text::new(txt)
        global::std_font()
        ]
    }
}

pub fn build() -> impl SceneList {
    bsn_list![
        head(),
        map(),
        info(format!("x:{: >5.0}\ny:{: >5.0}",0,0)),
        test::mesh2d(65536.0,Srgba::RED),
    ]
}
