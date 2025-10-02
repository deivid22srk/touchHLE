package org.touchhle.android;

import android.app.ActivityManager;
import android.content.Context;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Rect;
import android.os.Handler;
import android.os.Looper;
import android.util.AttributeSet;
import android.view.View;

public class PerformanceOverlayView extends View {
    private static final int UPDATE_INTERVAL_MS = 500;
    
    private Paint textPaint;
    private Paint backgroundPaint;
    private Handler handler;
    private Runnable updateRunnable;
    
    private boolean showFps = false;
    private boolean showRam = false;
    private int currentFps = 0;
    private float currentRamMB = 0.0f;
    
    private long frameCount = 0;
    private long lastFpsUpdateTime = 0;
    
    private ActivityManager activityManager;
    private ActivityManager.MemoryInfo memoryInfo;
    
    private int textSize = 40;
    private int padding = 20;
    
    public PerformanceOverlayView(Context context) {
        super(context);
        init(context);
    }
    
    public PerformanceOverlayView(Context context, AttributeSet attrs) {
        super(context, attrs);
        init(context);
    }
    
    private void init(Context context) {
        textPaint = new Paint(Paint.ANTI_ALIAS_FLAG);
        textPaint.setColor(Color.WHITE);
        textPaint.setTextSize(textSize);
        textPaint.setFakeBoldText(true);
        textPaint.setShadowLayer(4, 2, 2, Color.BLACK);
        
        backgroundPaint = new Paint();
        backgroundPaint.setColor(Color.argb(180, 0, 0, 0));
        
        activityManager = (ActivityManager) context.getSystemService(Context.ACTIVITY_SERVICE);
        memoryInfo = new ActivityManager.MemoryInfo();
        
        handler = new Handler(Looper.getMainLooper());
        updateRunnable = new Runnable() {
            @Override
            public void run() {
                updateMetrics();
                if (showFps || showRam) {
                    invalidate();
                    handler.postDelayed(this, UPDATE_INTERVAL_MS);
                }
            }
        };
    }
    
    public void setShowFps(boolean show) {
        this.showFps = show;
        updateVisibility();
    }
    
    public void setShowRam(boolean show) {
        this.showRam = show;
        updateVisibility();
    }
    
    private void updateVisibility() {
        if (showFps || showRam) {
            setVisibility(VISIBLE);
            startUpdating();
        } else {
            setVisibility(GONE);
            stopUpdating();
        }
    }
    
    private void startUpdating() {
        handler.removeCallbacks(updateRunnable);
        handler.post(updateRunnable);
    }
    
    private void stopUpdating() {
        handler.removeCallbacks(updateRunnable);
    }
    
    private void updateMetrics() {
        if (showFps) {
            calculateFps();
        }
        
        if (showRam) {
            calculateRam();
        }
    }
    
    private void calculateFps() {
        frameCount++;
        long currentTime = System.currentTimeMillis();
        
        if (lastFpsUpdateTime == 0) {
            lastFpsUpdateTime = currentTime;
        }
        
        long elapsedTime = currentTime - lastFpsUpdateTime;
        
        if (elapsedTime >= 1000) {
            currentFps = (int) ((frameCount * 1000.0f) / elapsedTime);
            frameCount = 0;
            lastFpsUpdateTime = currentTime;
        }
    }
    
    private void calculateRam() {
        if (activityManager != null) {
            activityManager.getMemoryInfo(memoryInfo);
            long usedMemory = memoryInfo.totalMem - memoryInfo.availMem;
            currentRamMB = usedMemory / (1024.0f * 1024.0f);
            
            Runtime runtime = Runtime.getRuntime();
            long appMemory = runtime.totalMemory() - runtime.freeMemory();
            currentRamMB = appMemory / (1024.0f * 1024.0f);
        }
    }
    
    @Override
    protected void onDraw(Canvas canvas) {
        super.onDraw(canvas);
        
        if (!showFps && !showRam) {
            return;
        }
        
        int yOffset = padding + 40;
        
        if (showFps) {
            String fpsText = String.format("FPS: %d", currentFps);
            Rect bounds = new Rect();
            textPaint.getTextBounds(fpsText, 0, fpsText.length(), bounds);
            
            int boxWidth = bounds.width() + padding * 4;
            int boxHeight = bounds.height() + padding * 3;
            
            canvas.drawRoundRect(
                padding,
                yOffset,
                padding + boxWidth,
                yOffset + boxHeight,
                16, 16,
                backgroundPaint
            );
            
            canvas.drawText(
                fpsText,
                padding * 2,
                yOffset + bounds.height() + padding * 1.5f,
                textPaint
            );
            
            yOffset += boxHeight + padding;
        }
        
        if (showRam) {
            String ramText = String.format("RAM: %.1f MB", currentRamMB);
            Rect bounds = new Rect();
            textPaint.getTextBounds(ramText, 0, ramText.length(), bounds);
            
            int boxWidth = bounds.width() + padding * 4;
            int boxHeight = bounds.height() + padding * 3;
            
            canvas.drawRoundRect(
                padding,
                yOffset,
                padding + boxWidth,
                yOffset + boxHeight,
                16, 16,
                backgroundPaint
            );
            
            canvas.drawText(
                ramText,
                padding * 2,
                yOffset + bounds.height() + padding * 1.5f,
                textPaint
            );
        }
    }
    
    @Override
    protected void onDetachedFromWindow() {
        super.onDetachedFromWindow();
        stopUpdating();
    }
}
