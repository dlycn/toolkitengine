use std::result;

const EFF_ZERO: u8 = 1 << 0;
const EFF_HALF: u8 = 1 << 1;
const EFF_ONE:  u8 = 1 << 2;
const EFF_TWO:  u8 = 1 << 3;

#[derive(Default,Clone,Debug)]
struct AttributeManager {
    pub attribute_max: u32,
    pub attribute_ini: Vec<Attribute>,
    pub attribute_task: Vec<(AttributeType, Vec<String>)>,
    pub attribute_judge: Vec<(AttributeJudge,u32,Vec<u32>)>,
}

/// * Attribute Standard
///
/// |TYPE    |SELF  |ATT METHOD |
/// |--------|------|-----------|
/// |Rule    |1.0   |pair       |
/// |Element |0.5   |ring-chain |
/// |Energy  |2.0   |pair       |
///
/// Bit layout:
///   EFF_ZERO = 1<<0 (0.0, immune)
///   EFF_HALF = 1<<1 (0.5, resisted / Element self)
///   EFF_ONE  = 1<<2 (1.0, neutral  / Rule self / initial)
///   EFF_TWO  = 1<<3 (2.0, super    / Energy self / Element super)
#[derive(Clone, Debug)]
enum AttributeType {
    Rule,
    Element,
    Energy,
}

#[derive(Clone, Debug)]
enum AttributeJudge {
    Super,
    Resisted,
    Immune,
}

#[derive(Debug,Clone)]
struct Attribute {
    pub attribute_name: String,
    pub attribute_type: AttributeType,
}

#[derive(Debug,Clone)]
struct AttributeBuilder {
    attribute_type: AttributeType,
    attribute_judge: AttributeJudge,
    attribute_root: AttributeManager,
}

impl AttributeBuilder {
    pub fn add(&mut self, attribute_names: Vec<&'static str>) -> &mut Self {
        self.attribute_root.attribute_max += attribute_names.len() as u32;
        self.attribute_root
            .attribute_task
            .push((self.attribute_type.clone(), attribute_names.iter().map(|s| s.to_string()).collect()));
        self
    }
    pub fn judge(
        &mut self,
        attribute_index: u32,
        attribute_names: Vec<u32>,
    ) -> &mut Self {
        self.attribute_root
            .attribute_judge
            .push((self.attribute_judge.clone(),attribute_index,attribute_names));
        self
    }
    pub fn backup(&self) -> AttributeManager {
        let opt = self.clone();
        opt.attribute_root
    }
}

impl AttributeManager {
    pub fn set(
        self,
        attribute_type: AttributeType,
        attribute_judge: AttributeJudge,
    ) -> AttributeBuilder {
        AttributeBuilder {
            attribute_type,
            attribute_judge,
            attribute_root: self,
        }
    }

    pub fn typematch(
        attribute_type: AttributeType,
    ) -> fn(Vec<Vec<u8>>, usize, usize) -> Vec<Vec<u8>> {
        match attribute_type {
            AttributeType::Rule => |mut data, index, num| {
                for i in 0..num {
                    data[index + i][index + i] = EFF_ONE;
                }
                data
            },
            AttributeType::Element => |mut data, index, num| {
                for i in 0..num {
                    for j in 0..num {
                        let v = if i == j {
                            EFF_HALF
                        } else if (j + num - i) % num == 1 {
                            EFF_TWO
                        } else {
                            EFF_HALF
                        };
                        data[index + i][index + j] = v;
                    }
                }
                data
            },
            AttributeType::Energy => |mut data, index, num| {
                for i in 0..num {
                    data[index + i][index + i] = EFF_TWO;
                }
                data
            },
        }
    }

    pub fn run(mut self) -> result::Result<String, String> {
        let n = self.attribute_max as usize;
        let row: Vec<u8> = std::iter::repeat(EFF_ONE).take(n).collect();
        let mut data: Vec<Vec<u8>> = std::iter::repeat(row).take(n).collect();
        let mut index: usize = 0;
        let mut num: usize = 0;
        for (attribute_type, attribute_names) in self.attribute_task {
            num += attribute_names.len();
            for attribute_name in attribute_names {
                self.attribute_ini.push(Attribute {
                    attribute_name,
                    attribute_type: attribute_type.clone(),
                });
            }
            data = AttributeManager::typematch(attribute_type)(data, index, num);
            index += num;
        }
        Ok(format!("{:?}\n{:?}", self.attribute_ini, data))
    }
}

fn main(){
    let attr = AttributeManager::default();
    let res = attr.set(AttributeType::Element, AttributeJudge::Super)
        .add(vec!["草","水","水"])
        .backup().run();
    println!("{}", res.unwrap());
}