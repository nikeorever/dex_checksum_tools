# Releasing

1. Update the `CHANGELOG.md`.

2. Update `Cargo.toml` with the new version.

3. Commit

   ```
   $ git commit -am "Prepare version X.Y.X"
   ```

4. Tag

   ```
   $ git tag -am "Version X.Y.Z" X.Y.Z
   ```

5. Push!

   ```
   $ git push && git push --tags
   ```
