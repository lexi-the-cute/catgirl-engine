package land.catgirl.engine;
    
import android.content.Context;
import android.content.res.AssetManager;
import android.os.Bundle;
import android.util.Log;

import org.libsdl.app.SDLActivity;

import java.io.BufferedInputStream;
import java.io.BufferedOutputStream;
import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;

public class Main extends SDLActivity {
    private static final String TAG = "CatgirlEngineApp";

    // This exists just to call SDLActivity
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Extract Default Assets
        try {
            extractDefaultAssets(getContext());
        } catch (IOException e) {
            Log.e(TAG, "IOException: Failed To Extract Assets Directory: " + e);
        }
    }

    private void extractDefaultAssets(Context context) throws IOException {
        // TODO: Enter Subdirectories
        File extracted = new File(context.getFilesDir(), "assets");
        AssetManager assetManager = context.getAssets();

        // Create Directory To Extract To
        if (!extracted.exists())
            extracted.mkdirs();

        String folder = "resourcepack";
        String[] fileList = assetManager.list(folder);

        Log.d(TAG, "Asset Count: " + fileList.length);
        for (String file : fileList) {
            File handle = new File(folder, file);
            Log.d(TAG, "Asset: " + handle.getPath());

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
//                Log.d(TAG, "Read: " + read);
            }

            bufferedInputStream.close();
            bufferedOutputStream.close();
        }
    }
}