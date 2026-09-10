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
    pub attribute_judge: Vec<(AttributeJudge,u8,Vec<u8>)>,
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
        attribute_index: u8,
        attribute_names: Vec<u8>,
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

    pub fn run(&self) -> result::Result<String, String> {
        self.backup()
        .run()
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
                if num == 1 { data[index + 0][index + 0] = EFF_HALF;} 
                else {
                for i in 0..num {
                    for j in 0..num {
                        let v = if i == j {
                            EFF_HALF
                        } else if num == 2{
                            EFF_HALF
                        } else if (j + num - i) % num == 1 {
                            EFF_TWO
                        } else {
                            EFF_HALF
                        };
                        data[index + i][index + j] = v;
                    }
                }}
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

    pub fn judgematch(attribute_judge: AttributeJudge) -> fn(Vec<Vec<u8>>, usize, Vec<u8>) -> Vec<Vec<u8>> {
        match attribute_judge {
            AttributeJudge::Super => |mut data, main_index, attribute_indexs| {
                for i in attribute_indexs {
                    data[main_index][i as usize] = EFF_TWO;
                }
                data
            },
            AttributeJudge::Resisted => |mut data, main_index, attribute_indexs| {
                for i in attribute_indexs {
                    data[main_index][i as usize] = EFF_HALF;
                }
                data
            },
            AttributeJudge::Immune => |mut data, main_index, attribute_indexs| {
                for i in attribute_indexs {
                    data[main_index][i as usize] = EFF_ZERO;
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
            num = attribute_names.len();
            for attribute_name in attribute_names {
                self.attribute_ini.push(Attribute {
                    attribute_name,
                    attribute_type: attribute_type.clone(),
                });
            }
            data = AttributeManager::typematch(attribute_type)(data, index, num);
            index += num;
        }
        for (attribute_judge, attribute_index, attribute_names) in self.attribute_judge {
            data = AttributeManager::judgematch(attribute_judge)(data, attribute_index as usize, attribute_names);
        }
        Ok(format!("{:?}\n{:?}", self.attribute_ini, data))
    }
}

fn main(){
    // Index: [0]草 [1]水 [2]火 | [3]电 [4]光 | [5]暗 [6]冰 | [7]地 [8]风
    let attr = AttributeManager::default();
    let res = attr.set(AttributeType::Element, AttributeJudge::Super)
        .add(vec!["草","水","火"])
        .add(vec!["电","光"])
        .backup()
        .set(AttributeType::Energy, AttributeJudge::Super)
        .add(vec!["暗","冰"])
        .backup()
        .set(AttributeType::Rule, AttributeJudge::Super)
        .add(vec!["地","风"])
        .judge(0, vec![4])   // 草→光=2.0
        .judge(2, vec![3,6])   // 火→电,冰=2.0
        .judge(3, vec![1,2])   // 电→水,火=2.0
        .judge(4, vec![5])   // 光→暗=2.0
        .judge(5, vec![0])   // 暗→草=2.0
        .judge(6, vec![7])   // 冰→地=2.0
        .judge(7, vec![3])   // 地→电=2.0
        .judge(8, vec![3])   // 风→电=2.0 (恒星风暴携带带电粒子)
        // === Resisted: 攻击方对目标微弱 ===
        .backup()
        .set(AttributeType::Rule, AttributeJudge::Resisted)
        .judge(0, vec![5,8])   // 草→暗,风=0.5
        .judge(1, vec![3])   // 水→电=0.5
        .judge(2, vec![7])   // 火→地=0.5
        .judge(3, vec![0])   // 电→草=0.5
        .judge(6, vec![2])   // 冰→火=0.5
        .judge(7, vec![1,5,6])   // 地→水,暗,冰=0.5
        .judge(8, vec![4,5,7])   // 风→光,暗,地=0.5
        // === Immune: 攻击方对目标无效 ===
        .backup()
        .set(AttributeType::Rule, AttributeJudge::Immune)
        .judge(3, vec![7])   // 电→地=0
        .judge(4, vec![0])   // 光→草=0
        .judge(7, vec![8])   // 地→风=0
        .run();
    println!("{}", res.unwrap());
}