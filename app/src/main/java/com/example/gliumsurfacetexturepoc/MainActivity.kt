package com.example.gliumsurfacetexturepoc

import android.graphics.SurfaceTexture
import android.opengl.GLSurfaceView
import android.os.Bundle
import android.util.Log
import android.view.TextureView
import androidx.appcompat.app.AppCompatActivity
import kotlin.properties.Delegates


class MainActivity : AppCompatActivity(), TextureView.SurfaceTextureListener {
    private var glSurfaceView: GLSurfaceView? = null
    private lateinit var surfaceTexture: SurfaceTexture
    private var width by Delegates.notNull<Int>()
    private var height by Delegates.notNull<Int>()

    companion object {
        private val LOG_TAG = MainActivity::class::simpleName.toString()
        external fun registerSurfaceTextureNativeHandler(surfaceTexture: SurfaceTexture): Boolean
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        glSurfaceView = findViewById( R.id.surfaceView )
    }

    override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
        surfaceTexture = surface
        this.width = width
        this.height = height
        registerSurfaceTextureNativeHandler(surfaceTexture)
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