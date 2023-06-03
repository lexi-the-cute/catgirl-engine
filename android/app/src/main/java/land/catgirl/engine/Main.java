package land.catgirl.engine;

import android.util.Log;
import android.os.Bundle;

import java.io.IOException;

import org.libsdl.app.SDLActivity;


public class Main extends SDLActivity {
    public static final String TAG = "CatgirlEngineApp";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Extract Default Assets
        try {
            Assets.extractDefaultAssets(getContext());
        } catch (IOException e) {
            Log.e(TAG, "IOException: Failed To Extract Assets Directory: " + e);
        }
    }
}