package com.example.gliumsurfacetexturepoc

import android.graphics.SurfaceTexture
import android.os.Bundle
import android.util.Log
import android.view.Surface
import android.view.TextureView
import androidx.appcompat.app.AppCompatActivity
import kotlin.properties.Delegates

class MainActivity : AppCompatActivity(), TextureView.SurfaceTextureListener {
    private var textureView: TextureView? = null
    private lateinit var surfaceTexture: SurfaceTexture
    private var width by Delegates.notNull<Int>()
    private var height by Delegates.notNull<Int>()

    companion object {
        init {
            System.loadLibrary("surface_texture_glium_c")
        }
        private val LOG_TAG = MainActivity::class::simpleName.toString()
        //external fun registerSurfaceTextureNativeHandler(surfaceTexture: SurfaceTexture, width: Int, height: Int): Boolean
        external fun initializeEGL(surface: Surface, surfaceTexture: SurfaceTexture, width: Int, height: Int): Boolean

    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        textureView = findViewById( R.id.textureView )
        textureView!!.setSurfaceTextureListener(this);
    }

    override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
        surfaceTexture = surface
        this.width = width
        this.height = height
        Log.d(LOG_TAG, "onSurfaceTextureAvailable: width: $width, height: $height")
        val surface = Surface(surfaceTexture)
        initializeEGL(surface, surfaceTexture, width, height)
        //registerSurfaceTextureNativeHandler(surfaceTexture, width, height)
    }

    override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) {
        surfaceTexture = surface
        this.width = width
        this.height = height
        Log.d(LOG_TAG, "surfaceTextureSizeChanged: width: $width, height: $height")
    }

    override fun onSurfaceTextureDestroyed(surface: SurfaceTexture): Boolean {
        Log.d(LOG_TAG, "onSurfaceTextureDestroyed");
        return true
    }

    override fun onSurfaceTextureUpdated(surface: SurfaceTexture) {
        Log.d(LOG_TAG, "onSurfaceTextureUpdated")
    }
}