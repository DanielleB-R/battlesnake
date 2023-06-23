(ns battlesnake.logic
  (:require [clojure.tools.logging :refer [info]]

            [battlesnake.geometry :refer [direction-adjacent]]))

(defn start-game [body]
  (info "Started game" body)
  nil)

(def initial-weights
  {:up 1.0
   :down 1.0
   :left 1.0
   :right 1.0})

(defn- avoid [weights direction]
  (assoc weights direction 0.0))

(defn- avoid-if-adjacent [weights head obstacle]
  (if-let [direction (direction-adjacent head obstacle)]
    (avoid weights direction)
    weights))

(defn- avoid-snake [weights {:keys [head]} {:keys [body]}]
  (reduce #(avoid-if-adjacent %1 head %2) weights body))

(defn- avoid-snakes [weights you snakes]
  (reduce #(avoid-snake %1 you %2) weights snakes))

(defn- avoid-myself [weights {:keys [head body]}]
  (reduce #(avoid-if-adjacent %1 head %2) weights (rest body)))

(defn- avoid-walls [weights {:keys [head]} {:keys [height width]}]
  (let [{:keys [x y]} head]
    (cond-> weights
      (= x 0) (avoid :left)
      (= x (dec width)) (avoid :right)
      (= y 0) (avoid :down)
      (= y (dec height)) (avoid :up))))

(defn- avoid-opponents [weights you snakes]
  (avoid-snakes weights you (remove #(= (:id %) (:id you)) snakes)))

(defn- choose-move [weights]
  ; Be very naive here for now
  (cond
    (pos? (:up weights)) "up"
    (pos? (:down weights)) "down"
    (pos? (:left weights)) "left"
    (pos? (:right weights)) "right"))

(defn make-move [{:keys [game turn board you]}]
  (info "Making move" (:id game) turn)
  (-> initial-weights
      (avoid-myself you)
      (avoid-walls you board)
      (avoid-opponents you (:snakes board))
      choose-move))

(defn end-game [body]
  (info "Ended game" body)
  nil)
