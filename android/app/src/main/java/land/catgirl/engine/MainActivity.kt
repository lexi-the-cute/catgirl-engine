package land.catgirl.engine

import android.os.Bundle
import android.util.Log
import android.view.WindowManager
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import com.google.androidgamesdk.GameActivity


class MainActivity : GameActivity() {
    val TAG: String = "CatgirlEngineApp";

    override fun onCreate(savedInstanceState: Bundle?) {
        // The GameActivity class creates a layout for you
        super.onCreate(savedInstanceState)

        Log.d(TAG, "Started Main Activity...")

        hideSystemUi()
    }

    private fun hideSystemUi() {
        Log.d(TAG, "Hiding System UI...")

        // Log.v(TAG, "Decor View: ${window.decorView}; Tag: ${window.decorView.tag}")
        val controller = WindowInsetsControllerCompat(window, window.decorView)

        // STATUS_BARS - Notification Icons Bar
        // NAVIGATION_BARS - The 3 buttons at the bottom
        // CAPTION_BAR - ???
        controller.hide(WindowInsetsCompat.Type.systemBars())

        // DISPLAY_CUTOUT - The area which extends past the normal screen
        // controller.hide(WindowInsetsCompat.Type.displayCutout())
        window.attributes.layoutInDisplayCutoutMode = WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES

        // The bar that says Catgirl Engine at the top
        supportActionBar?.hide()

        // Allow users to show bars by swiping
        controller.systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }

   companion object {
       init {
           System.loadLibrary("main")
       }
   }
}