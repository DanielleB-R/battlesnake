(ns battlesnake.geometry)

(defn coord [x y] {:x x :y y})

(defn direction-adjacent [origin adjacent]
  (cond
    (and (= (inc (:x origin)) (:x adjacent)) (= (:y origin) (:y adjacent))) :right
    (and (= (dec (:x origin)) (:x adjacent)) (= (:y origin) (:y adjacent))) :left
    (and (= (inc (:y origin)) (:y adjacent)) (= (:x origin) (:x adjacent))) :up
    (and (= (dec (:y origin)) (:y adjacent)) (= (:x origin) (:x adjacent))) :down
    :else nil))
