package com.wearos.videodashboard;

import android.app.Activity;
import android.net.Uri;
import android.os.Bundle;
import android.os.Handler;
import android.os.Looper;
import android.view.WindowManager;
import android.widget.VideoView;
import org.json.JSONObject;
import java.io.BufferedReader;
import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.util.Random;

public class MainActivity extends Activity {

    private VideoView videoView;
    private final String serverUrl = "http://dg08sgo8o8ws844cwsscccog.45.158.14.87.sslip.io";
    private final Handler handler = new Handler(Looper.getMainLooper());
    private int currentVersion = -1;
    private final Random random = new Random();

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON);
        setContentView(R.layout.activity_main);

        videoView = findViewById(R.id.videoView);
        videoView.setOnCompletionListener(mp -> videoView.start());

        startSync();
        startPixelShift();
    }

    private void startSync() {
        new Thread(() -> {
            try {
                URL url = new URL(serverUrl + "/static/version.json");
                HttpURLConnection conn = (HttpURLConnection) url.openConnection();
                BufferedReader reader = new BufferedReader(new InputStreamReader(conn.getInputStream()));
                StringBuilder sb = new StringBuilder();
                String line;
                while ((line = reader.readLine()) != null) sb.append(line);
                
                JSONObject json = new JSONObject(sb.toString());
                int newVersion = json.getInt("version");

                if (newVersion > currentVersion) {
                    downloadVideo();
                    currentVersion = newVersion;
                } else {
                    runOnUiThread(this::playLocalVideo);
                }
            } catch (Exception e) {
                e.printStackTrace();
                runOnUiThread(this::playLocalVideo);
            }
        }).start();
    }

    private void downloadVideo() {
        try {
            URL url = new URL(serverUrl + "/static/loop.mp4");
            HttpURLConnection conn = (HttpURLConnection) url.openConnection();
            InputStream is = conn.getInputStream();
            File file = new File(getFilesDir(), "loop.mp4");
            FileOutputStream os = new FileOutputStream(file);

            byte[] buffer = new byte[1024];
            int bytesRead;
            while ((bytesRead = is.read(buffer)) != -1) {
                os.write(buffer, 0, bytesRead);
            }
            os.close();
            is.close();

            runOnUiThread(this::playLocalVideo);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private void playLocalVideo() {
        File file = new File(getFilesDir(), "loop.mp4");
        if (file.exists()) {
            videoView.setVideoURI(Uri.fromFile(file));
            videoView.start();
        }
    }

    private void startPixelShift() {
        Runnable runnable = new Runnable() {
            @Override
            public void run() {
                float offsetX = random.nextInt(5) - 2; // -2 to +2
                float offsetY = random.nextInt(5) - 2;
                
                videoView.setTranslationX(offsetX);
                videoView.setTranslationY(offsetY);
                
                handler.postDelayed(this, 5 * 60 * 1000);
            }
        };
        handler.post(runnable);
    }
}
