# Hide left and right portion of status bar
set-option -g status-left " "
set-option -g status-right ""

# Active
set-window-option -g window-status-current-format "\
#[fg=default,bold, bg=default]#(despell -e #W)\
#[fg=default,      bg=default] #W "

# Inactive
set-window-option -g window-status-format "\
#[fg=default,dim, bg=default]#(despell -e #W)\
#[fg=default,     bg=default] #W "
