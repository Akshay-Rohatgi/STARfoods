current_time=$(date -u +%s)
last_4_digits=${current_time: -4}
echo $last_4_digits