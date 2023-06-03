package land.catgirl.engine;

import android.util.Log;
import android.content.Context;
import android.content.res.AssetManager;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.io.FileOutputStream;
import java.io.BufferedInputStream;
import java.io.BufferedOutputStream;

public class Assets {
    public static File getAssetsDirectory(Context context) {
        // TODO: Implement User Settings To Control Directory
        return new File(context.getFilesDir(), "assets");
    }

    public static void extractDefaultAssets(Context context) throws IOException {
        // TODO: Enter Subdirectories
        File extracted = getAssetsDirectory(context);
        AssetManager assetManager = context.getAssets();

        // Create Directory To Extract To
        if (!extracted.exists())
            extracted.mkdirs();

        String folder = "resourcepack";
        String[] fileList = assetManager.list(folder);

        Log.d(Main.TAG, "Asset Count: " + fileList.length);
        for (String file : fileList) {
            File handle = new File(folder, file);
            Log.d(Main.TAG, "Asset: " + handle.getPath());

            // TODO: Make This Efficient
            InputStream inputStream = assetManager.open(handle.getPath());
            BufferedInputStream bufferedInputStream = new BufferedInputStream(inputStream);

            File output = new File(extracted, file);  // TODO: Add Relative Paths Instead Of File Name
            FileOutputStream outputStream = new FileOutputStream(output);
            BufferedOutputStream bufferedOutputStream = new BufferedOutputStream(outputStream);

            int read;
            int offset = 0;
            byte[] buffer = new byte[4096];
            while ((read = bufferedInputStream.read(buffer)) > 0) {
                bufferedOutputStream.write(buffer, offset, read);
                bufferedOutputStream.flush();
                // Log.d(Main.TAG, "Read: " + read);
            }

            bufferedInputStream.close();
            bufferedOutputStream.close();
        }
    }
}