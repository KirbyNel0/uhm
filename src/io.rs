use std::{io::Read, sync::Arc};

pub struct FileDescriptor {
    pub name: String,
    pub content: String,
}

type Error = std::io::Error;

#[derive(Clone)]
pub enum ReadSource {
    Stdin,
    File(String),
}

impl ReadSource {
    pub fn is_stdin(&self) -> bool {
        if let Self::Stdin = self { true } else { false }
    }

    pub fn is_file(&self) -> bool {
        if let Self::File(_) = self {
            true
        } else {
            false
        }
    }

    pub fn map<'s, T>(&'s self, stream: T, file: impl FnOnce(&'s String) -> T) -> T {
        match self {
            Self::Stdin => stream,
            Self::File(f) => file(f),
        }
    }

    pub fn map_or<T>(&self, stream: impl FnOnce() -> T, file: impl FnOnce(&String) -> T) -> T {
        match self {
            Self::Stdin => stream(),
            Self::File(f) => file(f),
        }
    }

    pub fn read(&self) -> Result<String, Error> {
        match self {
            Self::Stdin => {
                let mut content = String::with_capacity(100);
                let success = std::io::stdin().read_to_string(&mut content);
                match success {
                    Ok(_) => Ok(content),
                    Err(e) => Err(e),
                }
            }
            Self::File(f) => std::fs::read_to_string(f),
        }
    }

    pub fn descriptor(self, content: String) -> FileDescriptor {
        match self {
            Self::Stdin => FileDescriptor {
                name: "[stdin]".to_string(),
                content,
            },
            Self::File(name) => FileDescriptor { name, content },
        }
    }

    pub fn read_descriptor(self) -> Result<FileDescriptor, Error> {
        match self.read() {
            Ok(content) => Ok(self.descriptor(content)),
            Err(e) => Err(e),
        }
    }

    pub fn read_descriptor_shared(self) -> Result<Arc<FileDescriptor>, Error> {
        match self.read() {
            Ok(content) => Ok(Arc::new(self.descriptor(content))),
            Err(e) => Err(e),
        }
    }
}

#[derive(Clone)]
pub enum WriteTarget {
    Stdout,
    File(String),
}

impl WriteTarget {
    pub fn is_stdout(&self) -> bool {
        if let Self::Stdout = self { true } else { false }
    }

    pub fn is_file(&self) -> bool {
        if let Self::File(_) = self {
            true
        } else {
            false
        }
    }

    pub fn map<'s, T>(&'s self, stream: T, file: impl FnOnce(&'s String) -> T) -> T {
        match self {
            Self::Stdout => stream,
            Self::File(f) => file(f),
        }
    }

    pub fn map_or<T>(&self, stream: impl FnOnce() -> T, file: impl FnOnce(&String) -> T) -> T {
        match self {
            Self::Stdout => stream(),
            Self::File(f) => file(f),
        }
    }

    pub fn write(&self, content: &str) -> Result<(), Error> {
        match self {
            Self::Stdout => {
                println!("{}", content);
                Ok(())
            }
            Self::File(f) => std::fs::write(f, content),
        }
    }

    pub fn descriptor(self, content: String) -> FileDescriptor {
        match self {
            Self::Stdout => FileDescriptor {
                name: "[stdout]".to_string(),
                content,
            },
            Self::File(name) => FileDescriptor { name, content },
        }
    }
}

impl Into<ReadSource> for WriteTarget {
    fn into(self) -> ReadSource {
        match self {
            WriteTarget::Stdout => ReadSource::Stdin,
            WriteTarget::File(f) => ReadSource::File(f),
        }
    }
}

impl Into<WriteTarget> for ReadSource {
    fn into(self) -> WriteTarget {
        match self {
            ReadSource::Stdin => WriteTarget::Stdout,
            ReadSource::File(f) => WriteTarget::File(f),
        }
    }
}
