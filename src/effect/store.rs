use bima_rs::cm::CM;
use bima_rs::record::Line;
use hdf5::{self, File, Group};
use pyo3::{PyErr, exceptions::PyValueError};
use std::fs::metadata;
use std::path::PathBuf;

pub struct Store {
    file: File,
    pub path: PathBuf,
    counters: Vec<usize>,
}

pub enum StoreErr {
    Hdf5Err(hdf5::Error),
    AlreadyExists,
}

impl From<StoreErr> for PyErr {
    fn from(value: StoreErr) -> Self {
        match value {
            StoreErr::AlreadyExists => PyValueError::new_err("File already exist"),
            StoreErr::Hdf5Err(e) => PyValueError::new_err(e.to_string()),
        }
    }
}

impl From<hdf5::Error> for StoreErr {
    fn from(e: hdf5::Error) -> Self {
        StoreErr::Hdf5Err(e)
    }
}

impl Store {
    pub fn new(
        path: PathBuf,
        n_objects: usize,
        redo: bool,
        save_acc: bool,
    ) -> Result<Self, StoreErr> {
        let file = File::create(&path)?;
        let metadata = metadata(&path).expect("Already check above, so must exist");
        if !redo && metadata.is_file() {
            return Err(StoreErr::AlreadyExists);
        }
        for obj_id in 0..n_objects {
            let obj_group = file.create_group(&format!("objects/{}", obj_id))?;
            obj_group.create_group("t")?;
            obj_group.create_group("x")?;
            obj_group.create_group("y")?;
            obj_group.create_group("z")?;
            obj_group.create_group("vx")?;
            obj_group.create_group("vy")?;
            obj_group.create_group("vz")?;
            if save_acc {
                obj_group.create_group("ax")?;
                obj_group.create_group("ay")?;
                obj_group.create_group("az")?;
            }
        }
        Ok(Store {
            file,
            path,
            counters: vec![0; n_objects],
        })
    }
    pub fn append(&mut self, obj_id: usize, lines: Vec<Line>, cm: &CM) -> hdf5::Result<()> {
        let chunck_id = self.counters[obj_id];
        self.counters[obj_id] += 1;
        let (t, x, y, z, vx, vy, vz, ax, ay, az) = unpack(lines, &cm);
        let obj_g = self.file.group(&format!("objects/{}", obj_id))?;
        store_dataset(&obj_g, "t", chunck_id, t)?;
        store_dataset(&obj_g, "x", chunck_id, x)?;
        store_dataset(&obj_g, "y", chunck_id, y)?;
        store_dataset(&obj_g, "z", chunck_id, z)?;
        store_dataset(&obj_g, "vx", chunck_id, vx)?;
        store_dataset(&obj_g, "vy", chunck_id, vy)?;
        store_dataset(&obj_g, "vz", chunck_id, vz)?;
        if let Some(a) = ax {
            store_dataset(&obj_g, "ax", chunck_id, a)?;
        }
        if let Some(a) = ay {
            store_dataset(&obj_g, "ay", chunck_id, a)?;
        }
        if let Some(a) = az {
            store_dataset(&obj_g, "az", chunck_id, a)?;
        }
        Ok(())
    }
}

fn store_dataset(obj_g: &Group, name: &str, chunck_id: usize, value: Vec<f64>) -> hdf5::Result<()> {
    let item = obj_g.group(name)?;
    item.new_dataset::<f64>()
        .create(chunck_id.to_string().as_str())?
        .write(value.as_slice())
}

fn unpack(
    lines: Vec<Line>,
    cm: &CM,
) -> (
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Vec<f64>,
    Option<Vec<f64>>,
    Option<Vec<f64>>,
    Option<Vec<f64>>,
) {
    let n = lines.len();

    let mut t = Vec::with_capacity(n);
    let mut x = Vec::with_capacity(n);
    let mut y = Vec::with_capacity(n);
    let mut z = Vec::with_capacity(n);
    let mut vx = Vec::with_capacity(n);
    let mut vy = Vec::with_capacity(n);
    let mut vz = Vec::with_capacity(n);

    // Pre-allocate optional acceleration vectors only if needed.
    let accel_exist = lines.first().and_then(|l| l.a).is_some();

    let (mut ax, mut ay, mut az) = if accel_exist {
        (
            Some(Vec::with_capacity(n)),
            Some(Vec::with_capacity(n)),
            Some(Vec::with_capacity(n)),
        )
    } else {
        (None, None, None)
    };

    for line in lines.into_iter() {
        t.push(line.t);

        x.push(line.r.x() + cm.x());
        y.push(line.r.y() + cm.y());
        z.push(line.r.z() + cm.z());

        vx.push(line.v.x());
        vy.push(line.v.y());
        vz.push(line.v.z());

        if let Some(a) = line.a {
            // safe: we only enter this branch when accel_exist = true
            ax.as_mut().unwrap().push(a.x());
            ay.as_mut().unwrap().push(a.y());
            az.as_mut().unwrap().push(a.z());
        }
    }

    (t, x, y, z, vx, vy, vz, ax, ay, az)
}
