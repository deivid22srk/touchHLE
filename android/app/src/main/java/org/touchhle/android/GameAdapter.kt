/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
package org.touchhle.android

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageButton
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView

class GameAdapter(
    private val games: List<GameInfo>,
    private val listener: OnGameClickListener
) : RecyclerView.Adapter<GameAdapter.GameViewHolder>() {
    
    interface OnGameClickListener {
        fun onGameClick(gameInfo: GameInfo)
        fun onGameMenuClick(gameInfo: GameInfo)
    }
    
    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): GameViewHolder {
        val view = LayoutInflater.from(parent.context)
            .inflate(R.layout.item_game, parent, false)
        return GameViewHolder(view)
    }
    
    override fun onBindViewHolder(holder: GameViewHolder, position: Int) {
        val game = games[position]
        holder.bind(game, listener)
    }
    
    override fun getItemCount(): Int = games.size
    
    class GameViewHolder(itemView: View) : RecyclerView.ViewHolder(itemView) {
        private val gameIconImageView: ImageView = itemView.findViewById(R.id.gameIconImageView)
        private val gameNameTextView: TextView = itemView.findViewById(R.id.gameNameTextView)
        private val gameVersionTextView: TextView = itemView.findViewById(R.id.gameVersionTextView)
        private val gameSizeTextView: TextView = itemView.findViewById(R.id.gameSizeTextView)
        private val statusIcon: ImageView = itemView.findViewById(R.id.statusIcon)
        private val statusTextView: TextView = itemView.findViewById(R.id.statusTextView)
        private val gameMenuButton: ImageButton = itemView.findViewById(R.id.gameMenuButton)
        
        fun bind(game: GameInfo, listener: OnGameClickListener) {
            gameNameTextView.text = game.name
            gameVersionTextView.text = game.version
            gameSizeTextView.text = game.size
            
            game.icon?.let { icon ->
                gameIconImageView.setImageBitmap(icon)
            } ?: run {
                gameIconImageView.setImageResource(R.drawable.icon)
            }
            
            when (game.type) {
                GameInfo.Type.IPA -> {
                    statusIcon.setImageResource(android.R.drawable.ic_media_play)
                    statusTextView.text = itemView.context.getString(R.string.tap_to_play)
                }
                GameInfo.Type.APP -> {
                    statusIcon.setImageResource(android.R.drawable.ic_media_play)
                    statusTextView.text = itemView.context.getString(R.string.tap_to_play)
                }
            }
            
            itemView.setOnClickListener {
                listener.onGameClick(game)
            }
            
            gameMenuButton.setOnClickListener {
                listener.onGameMenuClick(game)
            }
            
            itemView.isClickable = true
            itemView.isFocusable = true
        }
    }
}
