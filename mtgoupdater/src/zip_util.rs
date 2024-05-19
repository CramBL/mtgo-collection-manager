use std::{
    fs, io,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use zip::write::{ExtendedFileOptions, FileOptions};

const COMPRESSION_METHOD: zip::CompressionMethod = zip::CompressionMethod::BZIP2;
// Bzip2: 0 - 9. Default is 6
const COMPRESSION_LEVEL: i64 = 6;

/// Markers for the state of an archive
pub struct UnArchived;
pub struct Archived;

/// Marker for whether to delete a file or not
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum ShouldDelete {
    Yes,
    #[default]
    No,
}
pub struct Archive<State = UnArchived> {
    location: PathBuf,
    files: Vec<(PathBuf, ShouldDelete)>,
    _state: PhantomData<State>,
}

impl Archive<UnArchived> {
    /// Instantiates a new `Archive`, the archive is not created until `archive` is called, which stores the archive in the location specified by `location`
    pub fn new(location: impl AsRef<Path>) -> Self {
        Self {
            location: location.as_ref().to_path_buf(),
            files: Vec::new(),
            _state: PhantomData,
        }
    }

    /// Adds the given file to the archive while preserving the original file.
    pub fn add_file(&mut self, file: PathBuf) {
        self.files.push((file, ShouldDelete::No));
    }

    /// Moves the given file to the archive and deletes the original file.
    pub fn move_file(&mut self, file: PathBuf) {
        self.files.push((file, ShouldDelete::Yes));
    }

    /// Archives the files added to the `Archive` instance.
    ///
    /// # Returns
    /// An `Archive` instance with the state `Archived`.
    ///
    /// # Errors
    /// Returns an error if an error occurs while creating the archive.
    ///
    /// # Note
    /// Files that were added with `move_file` will be deleted after they have been moved to the archive.
    /// Files that were added with `add_file` will not be deleted.
    pub fn archive(self) -> Result<Archive<Archived>, std::io::Error> {
        let file = fs::File::create(&self.location)?;
        let mut zip = zip::ZipWriter::new(file);

        let options: FileOptions<_> = FileOptions::default()
            .compression_method(COMPRESSION_METHOD)
            .compression_level(Some(COMPRESSION_LEVEL));

        for (file, _) in &self.files {
            let filename: String = file
                .file_name()
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to get filename from path: {}",
                        file.to_string_lossy()
                    )
                })
                .to_str()
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to convert filename to string: {}",
                        file.to_string_lossy()
                    )
                })
                .to_owned();
            zip::ZipWriter::start_file::<_, ExtendedFileOptions, _>(
                &mut zip,
                filename,
                options.clone(),
            )?;
            let mut f = fs::File::open(file)?;
            io::copy(&mut f, &mut zip)?;
        }

        // Delete files that were moved to the archive
        for (file, should_delete) in &self.files {
            if *should_delete == ShouldDelete::Yes {
                fs::remove_file(file)?;
            }
        }

        zip.finish()?;

        Ok(Archive {
            location: self.location,
            files: self.files,
            _state: PhantomData,
        })
    }
}

impl Archive<Archived> {
    /// Instantiates a new `Archive` from an existing archive.
    ///
    /// In this case, the `Archive` instance is not used to create a new archive, but to add files to the existing archive.
    /// There's no knowledge of the files in the existing archive, querying the files in the archive will return an empty vector.
    ///
    /// # Arguments
    ///
    /// * `location` - The path to the existing archive
    pub fn init(location: impl AsRef<Path>) -> Self {
        Self {
            location: location.as_ref().to_path_buf(),
            files: Vec::with_capacity(0),
            _state: PhantomData,
        }
    }

    /// Returns the Path to the archive
    pub fn location(&self) -> &Path {
        &self.location
    }

    /// Adds the given files to the archive following the steps:
    /// 1. Add a new temporary archive at the same location as the existing archive
    /// 2. Copy the already compressed files from the existing archive to the temporary archive
    /// 3. Add the new files to the temporary archive
    /// 4. Delete the existing archive
    /// 5. Rename the temporary archive to the the name of the original archive
    pub fn add_to_archive<'f, F>(&mut self, new_files: F) -> Result<(), io::Error>
    where
        F: IntoIterator<Item = &'f Path>,
    {
        // 1. Add a new temporary archive at the same location as the existing archive
        let temp_archive_path = &self.location.with_file_name("temp.zip");
        let temp_archive = fs::File::create(temp_archive_path)?;

        let mut new_archive = zip::ZipWriter::new(temp_archive);

        // 2. Copy the already compressed files from the existing archive to the temporary archive
        let mut existing_archive = zip::ZipArchive::new(fs::File::open(&self.location)?)?;

        for i in 0..existing_archive.len() {
            let file = existing_archive.by_index(i)?;
            new_archive.raw_copy_file(file)?; // raw_copy_file preserves the compression method and level of the original file
        }

        let options: FileOptions<_> = FileOptions::default()
            .compression_method(COMPRESSION_METHOD)
            .compression_level(Some(COMPRESSION_LEVEL));

        // 3. Add the new files to the temporary archive
        for file in new_files {
            let filename: String = file
                .file_name()
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to get filename from path: {}",
                        file.to_string_lossy()
                    )
                })
                .to_str()
                .unwrap_or_else(|| {
                    panic!(
                        "Failed to convert filename to string: {}",
                        file.to_string_lossy()
                    )
                })
                .to_owned();
            new_archive.start_file::<_, ExtendedFileOptions, _>(filename, options.clone())?;
            let mut f = fs::File::open(file)?;
            io::copy(&mut f, &mut new_archive)?;
        }

        new_archive.finish()?;

        // 4. Delete the existing archive
        fs::remove_file(&self.location)?;

        // 5. Rename the temporary archive to the existing archive
        fs::rename(temp_archive_path, &self.location)?;

        Ok(())
    }

    /// Moves the given files to the archive and deletes the original files
    pub fn move_to_archive<'f, F>(&mut self, files: F) -> Result<(), io::Error>
    where
        F: IntoIterator<Item = &'f Path> + Clone,
    {
        // 1. Add the files to the archive
        self.add_to_archive(files.clone())?;

        // 2. Delete the files
        for file in files {
            fs::remove_file(file)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Read;
    use temp_dir::TempDir;

    /// Helper function to make a file with some contents at a given path
    fn create_file(path: &Path, contents: &str) {
        fs::write(path, contents).expect("Failed to write to file");
    }

    /// Helper function to check that the name and contents of a file in a zip archive are as expected
    fn check_zipfile_name_and_contents(
        expected_name: &str,
        expected_contents: &str,
        file: &mut zip::read::ZipFile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut zipped_file_contents = String::new();
        file.read_to_string(&mut zipped_file_contents)?;
        assert_eq!(
            expected_name,
            file.enclosed_name().unwrap().to_str().unwrap()
        );
        assert_eq!(expected_contents, zipped_file_contents);
        Ok(())
    }

    /// Compress a file and check that the archive contains the file
    /// Also check that the size of the archive is as expected
    #[test]
    fn test_archive_compression() {
        // Create a temporary directory
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");

        let file_contents: String = "1234567891011121314".repeat(10);
        let expect_unzipped_size: usize = file_contents.len();
        const EXPECT_ARCHIVE_SIZE: usize = 175;

        // Create a file in the temporary directory
        let temp_file = temp_dir.child("test.txt");
        fs::write(&temp_file, &file_contents).expect("Failed to write to temporary file");

        assert_eq!(
            file_contents,
            fs::read_to_string(&temp_file).expect("Failed to read from temporary file")
        );

        // Create an archive
        let child_path = temp_dir.child("test.zip");
        let mut archive = Archive::new(child_path);

        // Add the file to the archive
        archive.add_file(temp_file);

        // Archive the file
        let archived = archive.archive().expect("Failed to archive file");

        // Open the archive
        let file = fs::File::open(archived.location()).expect("Failed to open archive");

        let mut zip = zip::ZipArchive::new(file).expect("Failed to create ZipArchive");

        // Get the files from the archive
        let mut zipped_file = zip.by_index(0).expect("Failed to get file from archive");

        let mut zipped_file_contents = String::new();

        zipped_file
            .read_to_string(&mut zipped_file_contents)
            .expect("Failed to read file from archive");

        assert_eq!(file_contents, zipped_file_contents);

        // Size of zip
        let metadata = fs::metadata(archived.location()).expect("Failed to get metadata");
        assert_eq!(metadata.len(), EXPECT_ARCHIVE_SIZE as u64);
        eprintln!("Size of zip: {}", metadata.len());
        assert_eq!(zipped_file_contents.len(), expect_unzipped_size);
        eprintln!("unzipped size: {}", zipped_file_contents.len());
    }

    /// Compress the first two files, then add the third file to the archive and check that the archive contains all three files
    /// This test is to check that the `add_to_archive` method works
    #[test]
    fn tests_archive_add_to_archive() {
        // Files to compress
        let compress_files = [
            Path::new("Cargo.toml"),
            Path::new("src/lib.rs"),
            Path::new("README.md"),
        ];

        // Create a temporary directory
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        // Create child directory
        let child_dir = temp_dir.child("my_zip");

        // Zip cargo.toml and cargo.lock
        let mut archive = Archive::new(&child_dir);
        archive.add_file(compress_files[0].to_path_buf());
        archive.add_file(compress_files[1].to_path_buf());

        let mut archived = archive.archive().expect("Failed to archive file");

        // Then add the README.md to the existing archive
        archived
            .add_to_archive([compress_files[2]])
            .expect("Failed to add file to archive");

        // Check that the archive contains the files
        // Open the archive
        let mut zip =
            zip::ZipArchive::new(fs::File::open(child_dir).expect("failed to open archive file"))
                .expect("Failed to create ZipArchive");

        // Get the files from the archive and check file names and contents
        for (idx, compressed_file) in compress_files.iter().enumerate() {
            let mut zipped_file = zip.by_index(idx).expect("Failed to get file from archive");

            let mut zipped_file_contents = String::new();

            zipped_file
                .read_to_string(&mut zipped_file_contents)
                .expect("Failed to read file from archive");

            assert_eq!(
                zipped_file.enclosed_name().unwrap().to_str().unwrap(),
                compressed_file.file_name().unwrap()
            );
            assert_eq!(
                fs::read_to_string(compressed_file).expect("Failed to read file"),
                zipped_file_contents
            );
        }
    }

    /// Tests the `move_to_archive` method by:
    /// 1. Create a file f1 in dir_a and archive it.
    /// 2. Create 2 files f2 & f3 in dir_b.
    /// 3. Move f2 & f3 from dir_b to the archive in dir_a.
    /// 4. Check that the archive contains f1, f2, and f3 with correct name and content.
    /// 5. Check that f2 & f3 in dir_b are no longer there.
    #[test]
    fn test_move_to_archive() {
        let dir_a = TempDir::new().expect("Failed to create temporary directory");
        let dir_b = TempDir::new().expect("Failed to create temporary directory");

        // 1. Create a file in dir_a and archive it
        let (f1_name, f1_contents) = ("f1.txt", "f1 contents");
        let f1 = dir_a.child(f1_name);
        create_file(&f1, f1_contents);

        // Create an archive in dir_b and add f1 to it
        let mut archive = Archive::new(dir_a.child("archive.zip"));
        archive.add_file(f1.clone());
        let mut archived = archive.archive().expect("Failed to archive file");

        // 2. Create 2 files in dir_b
        let (f2_name, f2_contents) = ("f2.txt", "f2 contents");
        let f2 = dir_b.child(f2_name);
        create_file(&f2, f2_contents);

        let (f3_name, f3_contents) = ("f3.txt", "f3 contents");
        let f3 = dir_b.child(f3_name);
        create_file(&f3, f3_contents);

        // 3. Move f2 & f3 from dir_b to the archive in dir_a
        archived
            .move_to_archive([f2.as_path(), f3.as_path()])
            .expect("Failed to move files to archive");

        // 4. Check that the archive contains f1, f2, and f3 with correct name and content
        // Open the archive
        let mut zip = zip::ZipArchive::new(
            fs::File::open(archived.location()).expect("Failed to open archive file"),
        )
        .expect("Failed to create ZipArchive");

        // Get the files from the archive and check file names and contents
        check_zipfile_name_and_contents(
            f1_name,
            f1_contents,
            &mut zip.by_index(0).expect("Failed to get file from archive"),
        )
        .expect("Failed check file name and contents");

        check_zipfile_name_and_contents(
            f2_name,
            f2_contents,
            &mut zip.by_index(1).expect("Failed to get file from archive"),
        )
        .expect("Failed check file name and contents");

        check_zipfile_name_and_contents(
            f3_name,
            f3_contents,
            &mut zip.by_index(2).expect("Failed to get file from archive"),
        )
        .expect("Failed check file name and contents");

        // 5. Check that f2 & f3 in dir_b are no longer there
        assert!(!f2.exists());
        assert!(!f3.exists());
    }
}
