package land.catgirl.engine

import android.os.Bundle
import android.util.Log
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import com.google.androidgamesdk.GameActivity


class MainActivity : GameActivity() {
    val TAG: String = "CatgirlEngineApp";

    override fun onCreate(savedInstanceState: Bundle?) {
        // The GameActivity class creates a layout for you
        super.onCreate(savedInstanceState)

        Log.d(TAG, "Started Main Activity...")
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)

        if (hasFocus) {
            hideSystemUi()
        }
    }

    private fun hideSystemUi() {
        Log.d(TAG, "Hiding System UI...")

        val decorView = window.decorView
        val controller = WindowInsetsControllerCompat(
            window,
            decorView
        )

        // Log.v(TAG, "Decor View: $decorView; Tag: ${decorView.tag}")
        controller.hide(WindowInsetsCompat.Type.systemBars())
        controller.hide(WindowInsetsCompat.Type.displayCutout())
        controller.systemBarsBehavior =
            WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }

   companion object {
       init {
           System.loadLibrary("main")
       }
   }
}