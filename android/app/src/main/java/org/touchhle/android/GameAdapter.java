/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android;

import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ImageView;
import android.widget.TextView;
import android.widget.ImageButton;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;

import java.util.List;

public class GameAdapter extends RecyclerView.Adapter<GameAdapter.GameViewHolder> {
    
    public interface OnGameClickListener {
        void onGameClick(GameInfo gameInfo);
        void onGameMenuClick(GameInfo gameInfo);
    }
    
    private List<GameInfo> games;
    private OnGameClickListener listener;
    
    public GameAdapter(List<GameInfo> games, OnGameClickListener listener) {
        this.games = games;
        this.listener = listener;
    }
    
    @NonNull
    @Override
    public GameViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        View view = LayoutInflater.from(parent.getContext())
                .inflate(R.layout.item_game, parent, false);
        return new GameViewHolder(view);
    }
    
    @Override
    public void onBindViewHolder(@NonNull GameViewHolder holder, int position) {
        GameInfo game = games.get(position);
        holder.bind(game, listener);
    }
    
    @Override
    public int getItemCount() {
        return games.size();
    }
    
    static class GameViewHolder extends RecyclerView.ViewHolder {
        
        private ImageView gameIconImageView;
        private TextView gameNameTextView;
        private TextView gameVersionTextView;
        private TextView gameSizeTextView;
        private ImageView statusIcon;
        private TextView statusTextView;
        private ImageButton gameMenuButton;
        
        public GameViewHolder(@NonNull View itemView) {
            super(itemView);
            
            gameIconImageView = itemView.findViewById(R.id.gameIconImageView);
            gameNameTextView = itemView.findViewById(R.id.gameNameTextView);
            gameVersionTextView = itemView.findViewById(R.id.gameVersionTextView);
            gameSizeTextView = itemView.findViewById(R.id.gameSizeTextView);
            statusIcon = itemView.findViewById(R.id.statusIcon);
            statusTextView = itemView.findViewById(R.id.statusTextView);
            gameMenuButton = itemView.findViewById(R.id.gameMenuButton);
        }
        
        public void bind(GameInfo game, OnGameClickListener listener) {
            // Set game information
            gameNameTextView.setText(game.getName());
            gameVersionTextView.setText(game.getVersion());
            gameSizeTextView.setText(game.getSize());
            
            // Set game icon (for now use default, could extract from .ipa/.app later)
            gameIconImageView.setImageResource(R.drawable.icon);
            
            // Set status based on game type
            if (game.getType() == GameInfo.Type.IPA) {
                statusIcon.setImageResource(android.R.drawable.ic_media_play);
                statusTextView.setText(itemView.getContext().getString(R.string.tap_to_play));
            } else {
                statusIcon.setImageResource(android.R.drawable.ic_media_play);
                statusTextView.setText(itemView.getContext().getString(R.string.tap_to_play));
            }
            
            // Set click listeners
            itemView.setOnClickListener(v -> {
                if (listener != null) {
                    listener.onGameClick(game);
                }
            });
            
            gameMenuButton.setOnClickListener(v -> {
                if (listener != null) {
                    listener.onGameMenuClick(game);
                }
            });
            
            // Add ripple effect for better UX
            itemView.setClickable(true);
            itemView.setFocusable(true);
        }
    }
}