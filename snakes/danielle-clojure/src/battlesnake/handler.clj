(ns battlesnake.handler
  (:require [compojure.core :as w]
            [compojure.route :as route]
            [ring.middleware.defaults :refer [wrap-defaults api-defaults]]
            [ring.middleware.json :refer [wrap-json-response wrap-json-body]]
            [ring.util.response :refer [response]]
            [ring.logger :refer [wrap-with-logger]]

            [battlesnake.logic :refer [start-game make-move end-game]]))

(def snake-info
  {:apiversion "1"
   :author "DanielleB-R"
   :color "#116a0b"
   :head "default"
   :tail "default"
   :version "0.1.0"})

(w/defroutes app-routes
  (w/GET "/" [] (response snake-info))

  (w/POST "/start" request
    (let [body (:body request)]
      (start-game body)
      (response {})))

  (w/POST "/move" request
    (let [body (:body request)
          move (make-move body)]
      (response {:move move})))

  (w/POST "/end" request
    (let [body (:body request)]
      (end-game body)
      (response {})))

  (route/not-found "Not Found"))

(def app
  (-> app-routes
      (wrap-defaults api-defaults)
      (wrap-json-response)
      (wrap-json-body {:keywords? true})
      (wrap-with-logger)))
