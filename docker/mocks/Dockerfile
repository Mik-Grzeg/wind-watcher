FROM python:3.10.9
ARG MOCK_PATH

RUN pip install --root-user-action=ignore --upgrade pip
RUN echo ${MOCK_PATH}

RUN useradd -m api
USER api
WORKDIR /home/api
ENV PATH="/home/api/.local/bin:${PATH}"

COPY --chown=api:api ${MOCK_PATH}/requirements.txt requirements.txt
RUN pip install --user -r requirements.txt

COPY --chown=api:api ${MOCK_PATH}/ /home/api/

CMD gunicorn api:api --worker-class gevent -w 2 --log-level info --bind 0.0.0.0:8000
